#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct BotConfig {
    pub r#type: Option<String>,
    pub url: Option<String>,
    pub access_token: Option<String>,
    pub super_administrator: Vec<String>,
    pub bot_name: Option<String>,
    pub bot_id: Option<i64>,
}

impl Default for BotConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../config/botConfig.yml");
        let result: BotConfig =
            serde_yaml::from_str(yml_data).expect("[Bot] load config file fail");
        result
    }
}
