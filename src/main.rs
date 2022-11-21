use std::env;
use log::info;
use serde_json::{json, json_internal};
use RcoploBot_rs::core::event::{AddGroupRequest, GroupMemberDecrease, GroupMemberIncrease};
use RcoploBot_rs::core::WsClient;
use RcoploBot_rs::handler::api::osu_sb::OsuSbScores;
use RcoploBot_rs::service::CONTEXT;


#[tokio::main]
async fn main() {
    env::set_var("RUST_APP_LOG", "info");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");
    info!("Starting rbatis...");
    CONTEXT.init_pool().await;
    info!("Starting client...");
    WsClient::run().await;

    // let data = r#""#;
    // let result = serde_json::from_str::<GroupMemberIncrease>(data).unwrap();
    // println!("{:?}",result);
}