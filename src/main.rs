use std::env;
use log::info;
use serde_json::{json, json_internal};
use RcoploBot_rs::core::WsClient;
use RcoploBot_rs::handler::api::osu_sb::OsuSbScores;
use RcoploBot_rs::service::CONTEXT;
use RcoploBot_rs::util::image_osu_sb::image_osu_sb_pr;

#[tokio::main]
async fn main() {
    env::set_var("RUST_APP_LOG", "info");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");
    info!("Starting rbatis...");
    CONTEXT.init_pool().await;
    info!("Starting client...");
    WsClient::run().await;
}