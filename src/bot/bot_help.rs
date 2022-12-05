
use std::collections::HashMap;
use once_cell::sync::Lazy;
use rand::Rng;
use serde_json::{Map, Value};
use crate::core::bot::Bot;
use crate::core::event::Event;
use crate::bot::{AiHelp, SetuHelp};
use crate::core::group::Group;
use crate::util::file::{get_image_path, tmp_random_image_path};
use crate::util::regex_utils::contain;

pub static BOT_HELP: Lazy<HelpList> = Lazy::new(|| HelpList::default());

pub trait BotHelp {
    fn new() -> Help<'static>;
}
//一个简单的帮助模块,但是图还没画
#[derive(Debug,Clone)]
pub struct Help<'a>  {
    //模块名
    pub module_name: String,
    //模块简称
    pub module_name_abbreviation: String,
    //指令集合
    pub module_order: HashMap<&'a str,Vec<&'a str>>,
    //默认开关
    pub module_default: bool,
    //帮助详细信息
    pub module_help: Vec<&'a str>,
}
#[derive(Debug)]
pub struct HelpList<'a> {
    pub help: HashMap<String, Help<'a>>,
}
impl Default for HelpList<'_> {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("setu".to_string(), SetuHelp::new());
        map.insert("ai".to_string(), AiHelp::new());
        Self {
            help: map
        }
    }
}


pub async fn bot_help_group_handle(group:&mut Group) {
    if contain(&group.message,&vec!["/help","!help","！help"]){
        bot_help_group_image(group).await;
    }
}


pub async fn bot_help_group_image(group:&mut Group) {

}