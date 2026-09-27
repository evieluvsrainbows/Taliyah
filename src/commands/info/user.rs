use crate::{Context, Error, utils};
use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor},
    model::{
        Colour,
        guild::Member,
        user::{OnlineStatus, User}
    }
};

/// A set of commands for interacting with users or server members.
#[poise::command(slash_command, subcommands("age", "id", "info"))]
pub async fn user(_context: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Retrieves the user ID associated with the provided account. Defaults to your own account.
#[poise::command(slash_command, user_cooldown = 3)]
pub async fn id(context: Context<'_>, #[description = "The member whose account ID to retrieve."] member: Option<User>) -> Result<(), Error> {
    let author = context.author();
    let response = match member {
        Some(user) => format!("Hi **{}**, the user ID for **{}** is _{}_.", author.name, user.name, user.id),
        None => format!("Hello **{}**, your user ID is _{}_.", author.name, author.id)
    };
    context.say(response).await?;
    Ok(())
}

/// Retrieves the account age of the provided account. Defaults to your own account if nothing provided.
#[poise::command(slash_command, user_cooldown = 3)]
pub async fn age(
    context: Context<'_>,
    #[description = "The member whose account age to check."] member: Option<User>,
    #[description = "Whether or not to include the member's user id with the response."] with_user_id: Option<bool>
) -> Result<(), Error> {
    let author = context.author();
    let target = member.as_ref().unwrap_or(author);

    let created_at = target.created_at();
    let joined = created_at.format("%B %e, %Y");
    let joined_humanized = utils::time::humanize_duration(created_at.to_utc());

    let response = match member {
        None => format!("Hello **{}**, you joined Discord on {joined}, or _{joined_humanized}_.", author.name),
        Some(user) if with_user_id.unwrap_or(false) => format!(
            "Hi **{}**, {} (user id: _{}_) joined Discord on {joined}, or _{joined_humanized}_.",
            author.name, user.name, user.id
        ),
        Some(user) => format!("Hi **{}**, {} joined Discord on {joined}, or _{joined_humanized}_.", author.name, user.name)
    };

    context.say(response).await?;
    Ok(())
}

/// Retrieves detailed information about a user. Defaults to your own account if non-provided.
#[poise::command(slash_command, user_cooldown = 3, guild_only)]
pub async fn info(context: Context<'_>, #[description = "The member whose information to retrieve"] member: Option<Member>) -> Result<(), Error> {
    let member = match member {
        Some(m) => m,
        None => context.author_member().await.expect("Author member must exist in guild context").into_owned()
    };

    let user: &User = &member.user;
    let guild_id = context.guild_id().expect("Guild ID must exist");
    let cache = context.cache();

    let (active_status, activities_text, track_art, main_role, roles_display, role_count) = {
        let cached_guild = cache.guild(guild_id).expect("Guild missing from Serenity cache");

        let mut status_str = String::new();
        let mut act_str = String::new();
        let mut art_url = None;

        if let Some(presence) = cached_guild.presences.get(&user.id) {
            status_str.push_str(&format!("{} is currently ", user.name));

            let status_lbl = match presence.status {
                OnlineStatus::Online => "**Online**",
                OnlineStatus::Idle => "**Idle**",
                OnlineStatus::DoNotDisturb => "in **Do Not Disturb** mode",
                OnlineStatus::Invisible => "**Invisible**",
                _ => "**Offline**"
            };
            status_str.push_str(status_lbl);

            if let Some(cs) = &presence.client_status {
                let platform = match (cs.desktop.is_some(), cs.mobile.is_some(), cs.web.is_some()) {
                    (true, false, false) => "Desktop",
                    (false, true, false) => "Mobile",
                    (false, false, true) => "Web",
                    (true, true, false) => "Desktop and Mobile",
                    (true, true, true) => "Desktop, Mobile, and Web",
                    (false, true, true) => "Mobile and Web",
                    (true, false, true) => "Desktop and Web",
                    _ => ""
                };
                if !platform.is_empty() {
                    status_str.push_str(&format!(" on **{platform}**"));
                }
            }

            let parsed: Vec<_> = presence
                .activities
                .iter()
                .filter_map(|act| {
                    let (text, art) = utils::user_utils::parse_activity(act);
                    if art.is_some() {
                        art_url = art;
                    }
                    text
                })
                .collect();

            if !parsed.is_empty() {
                let activities_joined = parsed.join(" and ");
                let transition = if parsed.len() == 1 { " and " } else { ", " };
                act_str = format!("{transition}{activities_joined}.\n\n");
            } else {
                status_str.push_str(".\n\n");
            }
        }

        let main_role_str = match cached_guild.member_highest_role(&member) {
            Some(role) => format!("<@&{}>", role.id),
            None => "No main role available.".to_owned()
        };

        let mut member_roles: Vec<_> = member.roles.iter().filter_map(|role_id| cached_guild.roles.get(role_id)).collect();
        member_roles.sort_by_key(|r| r.position);
        member_roles.reverse();

        let count = member_roles.len();
        let roles_str = if member_roles.is_empty() {
            "No roles available.".to_owned()
        } else {
            member_roles.iter().map(|r| format!("<@&{}>", r.id)).collect::<Vec<_>>().join(" / ")
        };

        (status_str, act_str, art_url, main_role_str, roles_str, count)
    };

    let account_type = if user.bot { "Bot" } else { "User" };
    let created = user.created_at().format("%A, %B %e, %Y @ %l:%M %P");
    let joined = member
        .joined_at
        .map(|dt| dt.format("%A, %B %e, %Y @ %l:%M %P").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let nickname = member.nick.as_deref().unwrap_or("No nickname has been set.");
    let color = member.colour(cache).unwrap_or_else(|| Colour::new(0x00FF_FFFF));
    let hex = if member.colour(cache).is_none() {
        "No display color available.".to_owned()
    } else {
        format!("#{:06x}", color.0)
    };

    let mut embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(&user.name).icon_url(user.face()))
        .colour(color)
        .description(format!(
            "{active_status}{activities_text}\
            **__User Information__**:\n\
            **Type**: {account_type}\n\
            **Profile**: <@{id}>\n\
            **Tag**: {}\n\
            **ID**: {id}\n\
            **Creation Date**: {created}\n\n\
            **__Guild-related Information__**:\n\
            **Join Date**: {joined}\n\
            **Nickname**: {nickname}\n\
            **Display Color**: {hex}\n\
            **Main Role**: {main_role}\n\
            **Roles ({role_count})**: {roles_display}",
            user.tag(),
            id = user.id,
        ));

    if let Some(art) = track_art {
        embed = embed.thumbnail(art);
    }

    context.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
