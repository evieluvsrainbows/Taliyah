use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigurationData {
    pub bot: BotConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub general: GeneralConfig,
    pub database: DatabaseConfig,
    pub discord: DiscordConfig,
    pub denylist: DenylistConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub codename: String,
    pub prefix: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscordConfig {
    pub appid: u64,
    pub token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DenylistConfig {
    pub spotify: DenylistSpotifyConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DenylistSpotifyConfig {
    pub ids: Vec<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiConfig {
    pub entertainment: EntertainmentConfig,
    pub minecraft: MinecraftConfig,
    pub music: MusicConfig,
    pub services: ServicesConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntertainmentConfig {
    pub tmdb: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MinecraftConfig {
    pub hypixel: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MusicConfig {
    pub spotify: SpotifyConfig,
    pub lastfm: LastFmConfig,
    pub lavalink: LavalinkConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServicesConfig {
    pub github: String,
    pub google: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LastFmConfig {
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavalinkConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
}
