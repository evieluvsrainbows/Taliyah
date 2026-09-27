use crate::{Context, Error, utils};
use serenity::model::user::User;

/// A set of commands for interacting with users or server members.
#[poise::command(slash_command, subcommands("id", "age"))]
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
