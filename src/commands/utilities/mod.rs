use crate::{Context, Error};
use poise::command;

/// Shows the help menu.
#[command(slash_command, track_edits)]
pub async fn help(context: Context<'_>, #[description = "Specific command to show help about"] command: Option<String>) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration { ..Default::default() };
    poise::builtins::help(context, command.as_deref(), config).await?;
    Ok(())
}

/// Says hello to the user who initializes the command.
#[command(slash_command)]
pub async fn hello(context: Context<'_>) -> Result<(), Error> {
    context.say(format!("Hello, **{}**!", context.author().name)).await?;
    Ok(())
}

/// Posts a message containing a link to the bot's source code on GitHub.
#[command(slash_command)]
pub async fn source(context: Context<'_>) -> Result<(), Error> {
    context.reply("GitHub repository: <https://github.com/evelynharthbrooke/Taliyah>").await?;
    Ok(())
}
