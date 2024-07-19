use serenity::{
    all::{ActivityData, Context, EventHandler, OnlineStatus, Ready},
    async_trait
};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        let http = &context.http;

        let api_version = ready.version;
        let bot_gateway = http.get_bot_gateway().await.unwrap();
        let bot_owner = http.get_current_application_info().await.unwrap().owner.expect("Could not get owner!");
        let t_sessions = bot_gateway.session_start_limit.total;
        let r_sessions = bot_gateway.session_start_limit.remaining;

        info!("Successfully logged into Discord as the following user:");
        info!("Bot details: {} (User ID: {})", ready.user.tag(), ready.user.id);
        info!("Bot owner: {} (User ID: {})", bot_owner.tag(), bot_owner.id.to_string());

        let guild_count = ready.guilds.len();

        info!("Connected to the Discord API (version {api_version}) with {r_sessions}/{t_sessions} sessions remaining.");
        info!("Connected to and serving a total of {guild_count} guild(s).");

        let presence = format!("on {guild_count} guilds");
        context.set_presence(Some(ActivityData::playing(presence)), OnlineStatus::Online);
    }
}
