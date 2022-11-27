use std::collections::HashMap;
use log::info;
use rand::Rng;
use rbatis::rbdc::datetime::FastDateTime;
use crate::core::group::Group;
use crate::core::message::{at, text};
use crate::domain::SignGroupUsers;
use crate::bot::{ log_result};
use crate::bot::bot_help::{BOT_HELP, BotHelp, Help};
use crate::service::{CONTEXT, SignGroupUsersService};
use crate::util::regex_utils::{contain, rex_utils};


pub struct SignHelp;

impl BotHelp for SignHelp {
    fn new() -> Help<'static> {
        Help {
            module_name: "Sign 简易的签到功能".to_string(),
            module_name_abbreviation: "签到".to_string(),
            module_order: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("momo", vec!["{name}摸+", "摸+{name}","{at}摸+"]),
                ("good_feeling", vec!["{name}好感度", "好感度{name}","{at}好感度"]),
            ])),
            module_default: true,
            module_help: vec![
                "[摸+]{bot_name}",
                "{bot_name}[摸+]",
            ],
        }
    }
}

pub async fn sign_module_handle(group: &mut Group) {
    let sign_help = BOT_HELP.help.get("签到").unwrap();
    if contain(&group.message, sign_help.module_order.get("momo").unwrap()) {
        let sign2 = SignGroupUsersService::select_is_sign(&group.user_id, &group.group_id).await;
        match sign2 {
            None => {
                let sign = SignGroupUsersService::insert_sign(&group.user_id, &group.group_id).await;
                if sign {
                    let i = rand::thread_rng().gen_range(0..101);
                    let result = group.send_group_msg(vec![
                        at(&group.user_id),
                        text(" 喵喵~~签到成功了喵！\n"),
                        text(format!("心情值: {}", i).as_str()),
                    ]).await;
                    log_result(result);
                }
            }
            Some(data) => {
                let time = FastDateTime::now();
                let data_time = &data.checkin_count_last.unwrap();
                if (&time.get_day() == &data_time.get_day()) && (&time.get_mon() == &data_time.get_mon()) {
                    let result = group.send_group_msg(vec![
                        at(&group.user_id),
                        text(" 喵？今天你已经签到过了喵！"),
                    ]).await;
                    log_result(result);
                } else {
                    let sign1 = SignGroupUsersService::sign(&group.user_id, &group.group_id).await;
                    if sign1 {
                        let i = rand::thread_rng().gen_range(0..101);
                        let result = group.send_group_msg(vec![
                            at(&group.user_id),
                            text(" 喵喵~~签到成功了喵！\n"),
                            text(format!("心情值: {}", i).as_str()),
                        ]).await;
                        log_result(result);
                    }
                }
            }
        }
    } else if contain(&group.message, sign_help.module_order.get("good_feeling").unwrap()) {
        let sign2 = SignGroupUsersService::select_is_sign(&group.user_id, &group.group_id).await;
        match sign2 {
            None => {
                let result = group.send_group_msg(vec![
                    at(&group.user_id),
                    text(" 喵... 咱没有你的好感喵...,先摸摸小白吧~~ "),
                ]).await;
                log_result(result);
            }
            Some(data) => {
                let result = group.send_group_msg(vec![
                    at(&group.user_id),
                    text("咱对你的好感度为: "),
                    text(format!("{} 喵...", data.impression.unwrap()).as_str()),
                ]).await;
                log_result(result);
            }
        }
    }
}




