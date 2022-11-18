use log::info;
use rand::Rng;
use rbatis::rbdc::datetime::FastDateTime;
use crate::core::bot::Bot;
use crate::core::event::{Event, GroupMessageEvent};
use crate::core::message::{at, text};
use crate::domain::SignGroupUsers;
use crate::handler::{bot_name_compound, handle_frame};
use crate::handler::bot_help::{BotHelp, Help};
use crate::service::{CONTEXT, GroupFunctionService, SignGroupUsersService};
use crate::util::regex_utils::contain;

pub async fn sign_module_handle(event: &GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let open = GroupFunctionService::select_function_is_open(&event.group_id,"签到").await;
    if open {
        if contain(&event.raw_message,vec![
            bot_name_compound(0,"摸+").as_str(),
            bot_name_compound(1,"摸+").as_str(),
        ]){
            let sign2 = SignGroupUsersService::select_is_sign(&event.user_id, &event.group_id).await;
            info!("{:?}",sign2);
            match sign2 {
                None => {
                    let sign = SignGroupUsersService::insert_sign(&event.user_id, &event.group_id).await;
                    if sign {
                        let i = rand::thread_rng().gen_range(0..101);
                        let frame = bot.send_group_msg(event.group_id.clone(), vec![
                            at(&event.user_id),
                            text(" 喵喵~~签到成功了喵！\n"),
                            text(format!("心情值: {}", i).as_str()),
                        ]).await;
                        handle_frame(frame).await;

                    }
                }
                Some(data) => {
                    let time = FastDateTime::now();
                    let data_time = &data.checkin_count_last.unwrap();
                    if (&time.get_day() == &data_time.get_day())&&(&time.get_mon() == &data_time.get_mon()) {
                        let frame = bot.send_group_msg(event.group_id.clone(), vec![
                            at(&event.user_id),
                            text(" 喵？今天你已经签到过了喵！"),
                        ]).await;
                        handle_frame(frame).await;
                    }else {
                        let sign1 = SignGroupUsersService::sign(&event.user_id, &event.group_id).await;
                        if sign1 {
                            let i = rand::thread_rng().gen_range(0..101);
                            let frame = bot.send_group_msg(event.group_id.clone(), vec![
                                at(&event.user_id),
                                text(" 喵喵~~签到成功了喵！\n"),
                                text(format!("心情值: {}", i).as_str()),
                            ]).await;
                            handle_frame(frame).await;
                        }

                    }

                }
            }
        }else if contain(&event.raw_message,vec![
            bot_name_compound(0,"好感度").as_str(),
            bot_name_compound(1,"好感度").as_str(),
        ]) {
            let sign2 = SignGroupUsersService::select_is_sign(&event.user_id, &event.group_id).await;
            match sign2 {
                None => {
                    let frame = bot.send_group_msg(event.group_id.clone(), vec![
                        at(&event.user_id),
                        text(" 喵... 咱没有你的好感喵...,先摸摸小白吧~~ "),
                    ]).await;
                    handle_frame(frame).await;

                }
                Some(data) => {
                    let frame = bot.send_group_msg(event.group_id.clone(), vec![
                        at(&event.user_id),
                        text("咱对你的好感度为: "),
                        text(format!("{} 喵...",data.impression.unwrap()).as_str()),
                    ]).await;
                    handle_frame(frame).await;

                }
            }
        }
    }

}
pub struct SignHelp;
impl BotHelp for SignHelp {
    fn new() -> Help {
        Help{
            module_name: "Sign 简易的签到功能".to_string(),
            module_name_abbreviation: "签到".to_string(),
            module_default: true,
            module_help: vec![
                "[摸+][botName]".to_string(),
                "[botName][摸+]".to_string(),
                "中间无空格".to_string(),
            ]
        }
    }
}


