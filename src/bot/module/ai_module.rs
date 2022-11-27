use std::collections::HashMap;
use log::info;
use once_cell::sync::Lazy;
use rand::Rng;
use regex::RegexSet;
use url::form_urlencoded::parse;
use crate::core::group::Group;
use crate::core::message::text;
use crate::bot::bot_help::{BOT_HELP, BotHelp, Help};
use crate::service::{CONTEXT, SignGroupUsersService};
use crate::util::file::get_data_path;
use crate::util::regex_utils::{contain, contain_, rex_utils};
use std::default::Default;
use rbatis::rbdc::datetime::FastDateTime;
use crate::bot::log_result;
use crate::domain::SignGroupUsers;

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
    let (is, msg) = rex_utils(1,
                              &group.message,
                              None,
                              r"\w+",
                              None,
                              vec!["摸+", "好感度"]);

    if is {
        let sign2 = SignGroupUsersService::select_is_sign(&group.user_id, &group.group_id).await;
        match sign2 {
            None => {
                let result = group.send_group_msg(vec![text("喵... 你还没有签到喵...,要签了到才能互动喵!,摸摸小白吧~~")]).await;
                log_result(result);
            }
            Some(data) => {
                let time = FastDateTime::now();
                let data_time = &data.checkin_count_last.unwrap();
                if (&time.get_day() == &data_time.get_day()) && (&time.get_mon() == &data_time.get_mon()) {
                    let ai = &AI.to_vec();
                    let name = CONTEXT.bot_config.bot_name.as_ref().unwrap();
                    let regex_set = RegexSet::new(vec![format!(r"[{}]", msg)]).unwrap();
                    for data in ai {
                        if regex_set.is_match(data.r#type.as_str()) {
                            let i = data.data.len();
                            let i1 = rand::thread_rng().gen_range(0..i);
                            let result = group.send_group_msg(vec![text(data.data.get(i1).unwrap())]).await;
                            log_result(result);
                            return;
                        }
                    }
                } else {
                    let result = group.send_group_msg(vec![text("喵... 你还没有签到喵...,要签了到才能互动喵!,摸摸小白吧~~")]).await;
                    log_result(result);
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