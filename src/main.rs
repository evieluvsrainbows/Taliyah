use crate::serenity::GatewayIntents;
use listeners::handler::Handler;
use poise::serenity_prelude as serenity;
use poise::{builtins::register_application_commands_buttons, Framework, FrameworkOptions, PrefixFrameworkOptions};
use tracing::{info, Level};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use utils::read_config;

mod config;
mod listeners;
mod utils;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {}

#[poise::command(prefix_command, owners_only)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    register_application_commands_buttons(ctx).await?;
    Ok(())
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
            _ => Level::TRACE,
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
            commands: vec![register()],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(configuration.bot.general.prefix),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }))
        .build();

    let client = serenity::Client::builder(token, GatewayIntents::all()).event_handler(Handler).framework(framework).await;
    client.unwrap().start().await.unwrap();

    Ok(())
}
