
use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::core::bot::Bot;
use crate::core::event::Event;
use crate::handler::{SetuHelp, SignHelp};
use crate::util::ImageUtils;

pub static BOT_HELP: Lazy<HelpList> = Lazy::new(|| HelpList::default());

pub trait BotHelp {
    fn new() -> Help;
}

pub struct Help {
    //模块名
    pub module_name: String,
    //模块简称
    pub module_name_abbreviation: String,
    //默认开关
    pub module_default: bool,
    //帮助详细信息
    pub module_help: Vec<String>,
}

pub struct HelpList {
    pub help: HashMap<String, Help>,
}

pub async fn bot_help_handle(event: Event, bot: &mut Bot) {}

pub async fn bot_help_group_image() {}

impl Default for HelpList {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("setu".to_string(), SetuHelp::new());
        map.insert("签到".to_string(), SignHelp::new());
        Self {
            help: map
        }
    }
}