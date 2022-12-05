use std::fs::read_to_string;
use crate::config::ApplicationConfig;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct BotConfig {
    pub r#type: Option<String>,
    pub url: Option<String>,
    pub access_token: Option<String>,
    pub super_administrator: Vec<i64>,
    pub bot_name: Option<String>,
    pub bot_id: Option<i64>,
}

impl Default for BotConfig {
    fn default() -> Self {
        let yml_data =read_to_string("config/botConfig.yml").expect("[RcoploBot] load config file fail");
        let result = serde_yaml::from_str::<BotConfig>(&yml_data).unwrap();
        result
    }
}
