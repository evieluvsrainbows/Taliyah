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

        let version = ready.version;
        let gateway = http.get_bot_gateway().await.unwrap();
        let owner = http.get_current_application_info().await.unwrap().owner.expect("Could not get owner!");
        let total = gateway.session_start_limit.total;
        let remaining = gateway.session_start_limit.remaining;

        info!("Successfully logged into Discord as the following user:");
        info!("Bot details: {} (User ID: {})", ready.user.tag(), ready.user.id);
        info!("Bot owner: {} (User ID: {})", owner.tag(), owner.id.to_string());

        let guilds = ready.guilds.len();

        info!("Connected to the Discord API (version {version}) with {remaining}/{total} sessions remaining.");
        info!("Connected to and serving a total of {guilds} guild(s).");

        context.set_presence(Some(ActivityData::playing(format!("on {guilds} guilds"))), OnlineStatus::Online);
    }
}
