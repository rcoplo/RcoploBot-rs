use std::env;
use log::info;

use RcoploBot_rs::core::WsClient;
use RcoploBot_rs::handler::api::{get_lolicon, get_lolicon_r18_tag};
use RcoploBot_rs::service::CONTEXT;
use RcoploBot_rs::util::file::generate_random_tmp_image_path;

#[tokio::main]
async fn main() {
    env::set_var("RUST_APP_LOG","info");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");
    info!("Starting rbatis...");
    CONTEXT.init_pool().await;
    info!("Starting client...");
    WsClient::run().await;

}