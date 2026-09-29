#![allow(dead_code)]

mod commands;
mod config;
mod constants;
mod listeners;
mod models;
mod utils;

use config::ConfigurationData;
use constants::REQWEST_USER_AGENT;
use itertools::Itertools;
use listeners::handler::Handler;
use poise::{Framework, FrameworkOptions, serenity_prelude as serenity};
use reqwest::{Client, redirect::Policy};
use serenity::GatewayIntents;
use tracing::{Level, info};
use tracing_subscriber::EnvFilter;
use utils::read_config;

use std::time::Instant;

type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {
    config: ConfigurationData,
    reqwest_container: Client,
    start_time: Instant
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let configuration = read_config("config.toml");
    if configuration.bot.logging.enabled {
        let default_level = configuration.bot.logging.level.parse::<Level>().unwrap_or(Level::INFO);
        let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level.as_str()));
        tracing_subscriber::fmt().with_target(false).with_env_filter(env_filter).init();
        info!("Tracing initialized with default level {}", default_level);
    }

    let commands = vec![
        commands::info::bot::bot(),
        commands::info::user::user(),
        commands::moderation::moderation(),
        commands::search::tmdb::tmdb(),
    ];

    let token = configuration.bot.discord.token.clone();
    let config_clone = configuration.clone();

    let framework = Framework::builder()
        .options(FrameworkOptions { commands, ..Default::default() })
        .setup(move |context, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(context, &framework.options().commands).await?;
                let client = Client::builder().user_agent(REQWEST_USER_AGENT).redirect(Policy::none()).build()?;
                Ok(Data {
                    config: config_clone,
                    reqwest_container: client,
                    start_time: Instant::now()
                })
            })
        })
        .build();

    let options = framework.options();
    let command_count = options.commands.len();
    let subcommand_count: usize = options.commands.iter().map(|cmd| cmd.subcommands.len()).sum();

    let commands_str = options
        .commands
        .iter()
        .map(|cmd| {
            if cmd.subcommands.is_empty() {
                cmd.name.to_string()
            } else {
                let subs = cmd.subcommands.iter().map(|s| &s.name).join(", ");
                format!("{} ({subs})", cmd.name)
            }
        })
        .join(", ");

    info!("Initialized {} core commands with {} subcommands: {}", command_count, subcommand_count, commands_str);

    let mut client = serenity::Client::builder(token, GatewayIntents::all()).event_handler(Handler).framework(framework).await?;
    client.start_autosharded().await?;

    Ok(())
}
