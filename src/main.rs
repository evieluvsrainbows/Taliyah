#![allow(dead_code)]

mod commands;
mod config;
mod constants;
mod listeners;
mod models;
mod utils;

use config::ConfigurationData;
use constants::REQWEST_USER_AGENT;
use listeners::handler::Handler;
use poise::{serenity_prelude as serenity, Framework, FrameworkOptions};
use reqwest::{redirect::Policy, Client};
use serenity::GatewayIntents;
use tracing::{info, Level};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use utils::read_config;

type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {
    config: ConfigurationData,
    reqwest_container: Client
}

#[tokio::main(worker_threads = 16)]
async fn main() -> Result<(), Error> {
    let configuration = read_config("config.toml");
    if configuration.bot.logging.enabled {
        LogTracer::init()?;

        let level = match configuration.bot.logging.level.as_str() {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            _ => Level::TRACE
        };

        let subscriber = FmtSubscriber::builder()
            .with_target(false)
            .with_max_level(level)
            .with_env_filter(EnvFilter::from_default_env())
            .finish();

        tracing::subscriber::set_global_default(subscriber)?;

        info!("Tracing initialized with logging level set to {}.", level);
    }

    let token = configuration.bot.discord.token;
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                commands::fun::xkcd::xkcd(),
                commands::search::tmdb::tmdb(),
                commands::utilities::hello(),
                commands::utilities::help(),
                commands::utilities::source(),
            ],
            ..Default::default()
        })
        .setup(move |context, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(context, &framework.options().commands).await?;
                Ok(Data {
                    config: read_config("config.toml"),
                    reqwest_container: Client::builder().user_agent(REQWEST_USER_AGENT).redirect(Policy::none()).build()?
                })
            })
        })
        .build();

    let command_count = &framework.options().commands.len();
    let commands_str: String = framework.options().commands.iter().map(|c| &c.name).cloned().collect::<Vec<String>>().join(", ");
    info!("Initialized {} commands: {}", command_count, commands_str);

    let mut client = serenity::Client::builder(token, GatewayIntents::all()).event_handler(Handler).framework(framework).await?;
    if let Err(why) = client.start_autosharded().await {
        eprintln!("An error occurred while running the client: {why:?}");
    }

    Ok(())
}
