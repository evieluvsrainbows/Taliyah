use crate::serenity::GatewayIntents;
use poise::serenity_prelude as serenity;
use poise::{builtins::register_application_commands_buttons, Framework, FrameworkOptions, PrefixFrameworkOptions};
use utils::read_config;

mod config;
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
        .token(token)
        .intents(GatewayIntents::all())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();

    Ok(())
}
