use serenity::{
    all::{ActivityData, Context, EventHandler, OnlineStatus, Ready},
    async_trait
};
use tracing::{error, info};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        let http = &context.http;
        let guilds = ready.guilds.len();
        let (gateway_res, app_info_res) = tokio::join!(http.get_bot_gateway(), http.get_current_application_info());
        let gateway = match gateway_res {
            Ok(g) => g,
            Err(err) => {
                error!("Failed to fetch bot gateway: {err}");
                return;
            }
        };

        let app_info = match app_info_res {
            Ok(info) => info,
            Err(err) => {
                error!("Failed to fetch application info: {err}");
                return;
            }
        };

        let version = ready.version;
        let total = gateway.session_start_limit.total;
        let remaining = gateway.session_start_limit.remaining;

        info!("Successfully logged into the Discord API as the following user:");
        info!("Bot details: {} (User ID: {})", ready.user.tag(), ready.user.id);

        if let Some(owner) = app_info.owner {
            info!("Bot owner: {} (User ID: {})", owner.tag(), owner.id);
        } else {
            info!("Bot owner: [Team-owned application]");
        }

        info!("Connected to the Discord API (version {version}) with {remaining}/{total} sessions remaining.");
        info!("Connected to and serving a total of {guilds} guild(s).");

        let status_text = match guilds {
            1 => "on 1 guild".to_string(),
            _ => format!("on {guilds} guilds")
        };

        context.set_presence(Some(ActivityData::playing(status_text)), OnlineStatus::Online);
    }
}
