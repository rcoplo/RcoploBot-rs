
use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::core::bot::Bot;
use crate::core::event::Event;
use crate::handler::{AiHelp, OsuSbHelp, SetuHelp, SignHelp};
use crate::util::ImageUtils;

pub static BOT_HELP: Lazy<HelpList> = Lazy::new(|| HelpList::default());

pub trait BotHelp {
    fn new() -> Help<'static>;
}
#[derive(Debug)]
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

pub async fn bot_help_handle(event: Event, bot: &mut Bot) {}

pub async fn bot_help_group_image() {}

impl Default for HelpList<'_> {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("setu".to_string(), SetuHelp::new());
        map.insert("签到".to_string(), SignHelp::new());
        map.insert("ai".to_string(), AiHelp::new());
        map.insert("osusb".to_string(), OsuSbHelp::new());
        Self {
            help: map
        }
    }
}