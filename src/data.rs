use reqwest::Client as ReqwestClient;
use serenity::prelude::TypeMapKey;

pub struct ReqwestContainer;

impl TypeMapKey for ReqwestContainer {
    type Value = ReqwestClient;
}
