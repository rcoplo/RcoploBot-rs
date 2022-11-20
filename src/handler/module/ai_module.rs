use std::collections::HashMap;
use log::info;
use once_cell::sync::Lazy;
use rand::Rng;
use regex::RegexSet;
use url::form_urlencoded::parse;
use crate::core::group::Group;
use crate::core::message::text;
use crate::handler::{bot_name_compound, log_result};
use crate::handler::bot_help::{BOT_HELP, BotHelp, Help};
use crate::service::CONTEXT;
use crate::util::file::get_data_path;
use crate::util::regex_utils::{contain, contain_, rex_utils};
use std::default::Default;

pub static AI: Lazy<Vec<Ai>> = Lazy::new(|| Ai::new());

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Ai {
    pub r#type: String,
    pub data: Vec<String>,
}

pub struct AiHelp;

impl BotHelp for AiHelp {
    fn new() -> Help<'static> {
        Help {
            module_name: "ai".to_string(),
            module_name_abbreviation: "ai".to_string(),
            module_order: Default::default(),
            module_default: true,
            module_help: vec![
                "{bot_name}{msg}",
                "{msg}{bot_name}",
                "@bot {msg}",
            ],
        }
    }
}

pub async fn ai_group_module_handle(group: &mut Group) {
    let sign_help = BOT_HELP.help.get("签到").unwrap();
    let (is, msg) = rex_utils(4,
                              group.message.clone(),
                              None,
                              "(.*)",
                              None, );

    if is {
        if contain_(&group.message[0], sign_help.module_order.get("momo").unwrap()) {
            if contain_(&group.message[0], sign_help.module_order.get("good_feeling").unwrap()) {
                let ai = &AI.to_vec();
                let name = CONTEXT.bot_config.bot_name.as_ref().unwrap();
                let regex_set = RegexSet::new(vec![format!("[{}]", msg)]).unwrap();
                for data in ai {
                    if regex_set.is_match(data.r#type.as_str()) {
                        let i = data.data.len();
                        let i1 = rand::thread_rng().gen_range(0..i);
                        let result = group.send_group_msg(vec![text(data.data.get(i1).unwrap())]).await;
                        log_result(result);
                        return;
                    }
                }
            }
        }
    }
}

impl Ai {
    pub fn new() -> Vec<Self> {
        let data = include_str!("../../../resources/data/data.json");
        let ai: Vec<Ai> = serde_yaml::from_str(data).expect("[Bot] load config file fail");
        ai
    }
}