use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigurationData {
    pub bot: BotConfig,
    pub api: ApiConfig
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub general: GeneralConfig,
    pub discord: DiscordConfig,
    pub denylist: DenylistConfig,
    pub logging: LoggingConfig
}

#[derive(Deserialize)]
pub struct GeneralConfig {
    pub codename: String
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String
}

#[derive(Deserialize)]
pub struct DiscordConfig {
    pub appid: u64,
    pub token: String
}

#[derive(Deserialize)]
pub struct DenylistConfig {
    pub spotify: DenylistSpotifyConfig
}

#[derive(Deserialize)]
pub struct DenylistSpotifyConfig {
    pub ids: Vec<u64>
}

#[derive(Deserialize)]
pub struct ApiConfig {
    pub entertainment: EntertainmentConfig,
    pub music: MusicConfig,
    pub services: ServicesConfig
}

#[derive(Deserialize)]
pub struct EntertainmentConfig {
    pub tmdb: String
}

#[derive(Deserialize)]
pub struct MusicConfig {
    pub spotify: SpotifyConfig,
    pub lastfm: LastFmConfig
}

#[derive(Deserialize)]
pub struct ServicesConfig {
    pub github: String,
    pub google: String
}

#[derive(Deserialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String
}

#[derive(Deserialize)]
pub struct LastFmConfig {
    pub api_key: String
}
