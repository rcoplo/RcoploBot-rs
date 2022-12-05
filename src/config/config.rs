use std::fs::read_to_string;
use log::info;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct ApplicationConfig {
    pub debug: bool,
    pub database_url: String,
    pub bili_cookie: BiliCookieConfig,
    pub setu: SetuConfig,
    pub api: ApiConfig,
    pub search_image: SearchImageConfig,
    pub osu_api_v2: OsuApiV2Config,
    pub osu_api_v1: OsuApiV1Config,
    pub sign_in: SignInConfig,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct BiliCookieConfig {
    pub SESSDATA: String,
    pub bili_jct: String,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct SetuConfig {
    pub local_storage: bool,
    pub withdraw_time: u64,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct ApiConfig {
    pub amap_api_key: String,
    pub qweather_api_key: String,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct SearchImageConfig {
    pub max_find_image_count: i32,
    pub saucenao_api_key: String,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct OsuApiV2Config {
    pub client_id: i32,
    pub client_secret: String,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct OsuApiV1Config {
    pub api_key: String,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub struct SignInConfig {
    pub sign_scope: f64,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let yml_data =read_to_string("config/application.yml").expect("[RcoploBot] load config file fail");
        let result = serde_yaml::from_str::<ApplicationConfig>(&yml_data).unwrap();
        if result.debug {
            info!("[RcoploBot] load config:{:?}", result);
            info!("[RcoploBot] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            info!("[RcoploBot] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}