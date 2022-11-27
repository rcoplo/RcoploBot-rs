use std::env;
use std::fs::File;
use std::io::BufReader;
use image::ImageFormat;
use log::info;
use plotters::backend::{BitMapBackend, DrawingBackend};
use plotters::element::BackendCoordAndZ;
use plotters::prelude::{BitMapElement, IntoTextStyle};
use plotters::style::{ RED, TextStyle};
use rand::Rng;
use serde_json::{json, json_internal};
use RcoploBot_rs::core::event::{AddGroupRequest, GroupMemberDecrease, GroupMemberIncrease};
use RcoploBot_rs::core::WsClient;
use RcoploBot_rs::bot::api::osu_sb::OsuSbScores;
use RcoploBot_rs::service::CONTEXT;
use RcoploBot_rs::util::file::{get_image_path, get_tmp_path, tmp_random_image_path};
use RcoploBot_rs::util::sign_image;


#[tokio::main]
async fn main() {
    // env::set_var("RUST_APP_LOG", "info");
    // pretty_env_logger::init_custom_env("RUST_APP_LOG");
    // info!("Starting rbatis...");
    // CONTEXT.init_pool().await;
    // info!("Starting client...");
    // WsClient::run().await;
    sign_image();
}

