use crate::{Context, Error, utils};
use poise::{CreateReply, serenity_prelude::*};

/// A set of commands for retrieving information about the bot.
#[poise::command(slash_command, subcommands("about"))]
pub async fn bot(_context: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Retrieves information about the bot.
#[poise::command(slash_command, user_cooldown = 3)]
pub async fn about(context: Context<'_>) -> Result<(), Error> {
    let bot_user = context.cache().current_user().clone();
    let name = &bot_user.name;
    let avatar = bot_user.face();
    let user_id = bot_user.id;

    let guilds = context.cache().guild_count();
    let users = context.cache().user_count();

    let uptime = utils::time::format_uptime(context.data().start_time);
    let version = env!("CARGO_PKG_VERSION");

    let embed = CreateEmbed::new()
        .color(Color::BLURPLE)
        .author(CreateEmbedAuthor::new(name).icon_url(avatar))
        .field("Uptime", uptime, true)
        .field("\u{200B}", "\u{200B}", true)
        .field("Version", version, true)
        .field("Users", users.to_string(), true)
        .field("\u{200B}", "\u{200B}", true)
        .field("Guilds", guilds.to_string(), true)
        .footer(CreateEmbedFooter::new(format!("{name} user ID: {user_id}")));

    let builder = CreateReply::default().embed(embed);
    context.send(builder).await?;

    Ok(())
}
