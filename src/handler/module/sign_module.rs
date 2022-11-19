use log::info;
use rand::Rng;
use rbatis::rbdc::datetime::FastDateTime;
use crate::core::group::Group;
use crate::core::message::{at, text};
use crate::domain::SignGroupUsers;
use crate::handler::{bot_name_compound, log_result};
use crate::handler::bot_help::{BotHelp, Help};
use crate::service::{CONTEXT, SignGroupUsersService};
use crate::util::regex_utils::contain;

pub async fn sign_module_handle(group: &mut Group) {
    if contain(&group.raw_message, &vec![
        bot_name_compound(0, "摸+").as_str(),
        bot_name_compound(1, "摸+").as_str(),
    ]) {
        let sign2 = SignGroupUsersService::select_is_sign(&group.user_id, &group.group_id).await;
        info!("{:?}",sign2);
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
    } else if contain(&group.raw_message, &vec![
        bot_name_compound(0, "好感度").as_str(),
        bot_name_compound(1, "好感度").as_str(),
    ]) {
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

pub struct SignHelp;

impl BotHelp for SignHelp {
    fn new() -> Help<'static> {
        Help {
            module_name: "Sign 简易的签到功能".to_string(),
            module_name_abbreviation: "签到".to_string(),
            module_order: Default::default(),
            module_default: true,
            module_help: vec![
                "[摸+][botName]",
                "[botName][摸+]",
                "中间无空格",
            ],
        }
    }
}


