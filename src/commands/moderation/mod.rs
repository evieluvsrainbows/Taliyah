use crate::{Context, Error};
use serenity::builder::{AutocompleteChoice, CreateAutocompleteResponse, EditChannel};
use serenity::model::{guild::Member, id::UserId};

/// A set of commands related to moderating the server.
#[poise::command(slash_command, guild_only, subcommands("ban", "unban", "kick", "slowmode"))]
pub async fn moderation(_context: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Bans someone from the server.
#[poise::command(slash_command, guild_only, required_permissions = "BAN_MEMBERS", required_bot_permissions = "BAN_MEMBERS")]
pub async fn ban(
    context: Context<'_>,
    #[description = "The member to ban."] member: Option<Member>,
    #[description = "The user ID of the person you are banning. Alternative to member parameter."] user_id: Option<UserId>,
    #[description = "The reason why the member is being banned. Optional."] reason: Option<String>
) -> Result<(), Error> {
    let guild_id = context.guild_id().unwrap();
    let (target_id, target_name) = match (member, user_id) {
        (Some(m), _) => (m.user.id, m.user.name.clone()),
        (None, Some(uid)) => {
            let name = uid.to_user(context.http()).await.map(|u| u.name).unwrap_or_else(|_| uid.to_string());
            (uid, name)
        }
        (None, None) => {
            context.say("Please provide either a member or a user ID to ban.").await?;
            return Ok(());
        }
    };

    if let Some(ref r) = reason {
        guild_id.ban_with_reason(context.http(), target_id, 0, r).await?;
        context.say(format!("Member **{}** (id: `{}`) has been banned for: **_{}_**", target_name, target_id, r)).await?;
    } else {
        guild_id.ban(context.http(), target_id, 0).await?;
        context.say(format!("Member **{}** (id: `{}`) has been banned.", target_name, target_id)).await?;
    }

    Ok(())
}

/// Unbans someone from the server.
#[poise::command(slash_command, guild_only, required_permissions = "BAN_MEMBERS", required_bot_permissions = "BAN_MEMBERS")]
pub async fn unban(context: Context<'_>, #[description = "The user ID of the person to unban."] user_id: UserId) -> Result<(), Error> {
    let guild_id = context.guild_id().unwrap();
    let target_name = user_id.to_user(context.http()).await.map(|u| u.name).unwrap_or_else(|_| user_id.to_string());
    guild_id.unban(context.http(), user_id).await?;
    context.say(format!("User **{}** with id `{}` has been unbanned.", target_name, user_id)).await?;
    Ok(())
}

/// Kicks someone from the server.
#[poise::command(slash_command, guild_only, required_permissions = "KICK_MEMBERS", required_bot_permissions = "KICK_MEMBERS")]
pub async fn kick(
    context: Context<'_>,
    #[description = "The member to kick."] member: Member,
    #[description = "The reason why the member is being kicked. Optional."] reason: Option<String>
) -> Result<(), Error> {
    if let Some(reason) = reason {
        member.kick_with_reason(context.http(), &reason).await?;
        context
            .say(format!("Member **{}** with id `{}` has been kicked for: **_{reason}_**", member.user.name, member.user.id))
            .await?;
    } else {
        member.kick(context.http()).await?;
        context.say(format!("Member **{}** with id `{}` has been kicked.", member.user.name, member.user.id)).await?;
    }
    Ok(())
}

async fn autocomplete_seconds(_context: Context<'_>, partial: &str) -> CreateAutocompleteResponse {
    let choices = [
        ("Off (0s)", 0u16),
        ("5 seconds", 5),
        ("10 seconds", 10),
        ("15 seconds", 15),
        ("30 seconds", 30),
        ("1 minute (60s)", 60),
        ("2 minutes (120s)", 120),
        ("5 minutes (300s)", 300),
        ("10 minutes (600s)", 600),
        ("15 minutes (900s)", 900),
        ("30 minutes (1800s)", 1800),
        ("1 hour (3600s)", 3600),
        ("2 hours (7200s)", 7200),
        ("6 hours (21600s)", 21600)
    ];

    let filtered_choices = choices
        .iter()
        .filter(|(name, val)| name.to_lowercase().contains(&partial.to_lowercase()) || val.to_string().starts_with(partial))
        .map(|&(name, val)| AutocompleteChoice::new(name, val));

    CreateAutocompleteResponse::new().set_choices(filtered_choices.collect())
}

/// Sets the slowmode rate for the current channel.
#[poise::command(slash_command, guild_only, required_permissions = "MANAGE_CHANNELS")]
pub async fn slowmode(
    context: Context<'_>,
    #[description = "The slowmode rate in seconds (leave empty to view current rate)"]
    #[autocomplete = autocomplete_seconds]
    seconds: Option<u16>
) -> Result<(), Error> {
    let channel_id = context.channel_id();
    let response = if let Some(slowmode_rate) = seconds {
        let builder = EditChannel::new().rate_limit_per_user(slowmode_rate);
        if let Err(why) = channel_id.edit(context.http(), builder).await {
            tracing::error!("Error setting channel slowmode rate: {:?}", why);
            format!("Failed to set slowmode to `{slowmode_rate}` seconds.")
        } else if slowmode_rate == 0 {
            "I have disabled slowmode for this channel. Server members can now send messages without waiting.".to_string()
        } else {
            format!("Successfully set the channel slowmode rate to **{slowmode_rate}** seconds.")
        }
    } else {
        let channel = channel_id.to_channel(context.http()).await?;
        if let Some(channel) = channel.guild() {
            match channel.rate_limit_per_user {
                Some(rate) => {
                    if rate == 0 {
                        "Slowmode is currently disabled for this channel.".to_string()
                    } else {
                        format!("The slowmode rate for this channel is currently set to **{rate}** seconds.")
                    }
                }
                None => "Slowmode is not available for this channel type.".to_string()
            }
        } else {
            "Failed to find channel in cache.".to_string()
        }
    };

    context.say(response).await?;

    Ok(())
}
