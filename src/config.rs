use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigurationData {
    pub bot: BotConfig,
    pub api: ApiConfig
}

#[derive(Deserialize, Debug, Clone)]
pub struct BotConfig {
    pub discord: DiscordConfig,
    pub logging: LoggingConfig
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct DiscordConfig {
    pub appid: u64,
    pub token: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiConfig {
    pub entertainment: EntertainmentConfig
}

#[derive(Deserialize, Debug, Clone)]
pub struct EntertainmentConfig {
    pub tmdb: String
}
