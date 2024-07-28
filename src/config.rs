use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigurationData {
    pub bot: BotConfig,
    pub api: ApiConfig
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub general: GeneralConfig,
    pub database: DatabaseConfig,
    pub discord: DiscordConfig,
    pub denylist: DenylistConfig,
    pub logging: LoggingConfig
}

#[derive(Clone, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub codename: String,
    pub prefix: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DiscordConfig {
    pub appid: u64,
    pub token: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DenylistConfig {
    pub spotify: DenylistSpotifyConfig
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DenylistSpotifyConfig {
    pub ids: Vec<u64>
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ApiConfig {
    pub entertainment: EntertainmentConfig,
    pub minecraft: MinecraftConfig,
    pub music: MusicConfig,
    pub services: ServicesConfig
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EntertainmentConfig {
    pub tmdb: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MinecraftConfig {
    pub hypixel: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MusicConfig {
    pub spotify: SpotifyConfig,
    pub lastfm: LastFmConfig,
    pub lavalink: LavalinkConfig
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ServicesConfig {
    pub github: String,
    pub google: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LastFmConfig {
    pub api_key: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LavalinkConfig {
    pub host: String,
    pub port: u16,
    pub password: String
}
