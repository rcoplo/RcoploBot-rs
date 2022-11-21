use std::fmt::Display;
use log::{error, info, warn};
use serde_json::Value;

use crate::core::bot::{Bot, ResultFrame};
use crate::core::event::*;
use crate::core::friend::Friend;
use crate::core::group::Group;
use crate::core::notice::{Notice};
use crate::core::request::Request;

use crate::domain::{GroupFunction, Setu};
use crate::handler::*;
use crate::service::{CONTEXT, GroupFunctionService, SetuService};
use crate::util::regex_utils::contain;

pub async fn event_handle(event: Event, bot: &mut Bot) {
    let group_id = match &event {
        Event::FriendMessageEvent(event) => {
            None
        }
        Event::GroupMessageEvent(event) => {
            Some(event.group_id)
        }
        Event::GroupFileUpload(event) => {
            Some(event.group_id)
        }
        Event::GroupAdminChange(event) => {
            Some(event.group_id)
        }
        Event::GroupMemberDecrease(event) => {
            Some(event.group_id)
        }
        Event::GroupMemberIncrease(event) => {
            Some(event.group_id)
        }
        Event::GroupBan(event) => {
            Some(event.group_id)
        }
        Event::FriendAdd(event) => {
            None
        }
        Event::GroupMessageRecall(event) => {
            Some(event.group_id)
        }
        Event::FriendMessageRecall(event) => {
            None
        }
        Event::FriendPoke(event) => {
            None
        }
        Event::GroupPoke(event) => {
            Some(event.group_id)
        }
        Event::TipsForLuckyKingOfRedPackets(event) => {
            Some(event.group_id)
        }
        Event::GroupMemberHonorChangePrompt(event) => {
            Some(event.group_id)
        }
        Event::GroupMemberBusinessCardUpdate(event) => {
            Some(event.group_id)
        }
        Event::OfflineFileReceived(event) => {
            None
        }
        Event::AddFriendRequest(event) => {
            None
        }
        Event::AddGroupRequest(event) => {
            Some(event.group_id)
        }
        Event::OtherClientOnlineStatusChanges(event) => {
            None
        }
        Event::ApiResult(event) => {
            None
        }
        Event::EssenceMessage(event) => {
            None
        }

    };
    match group_id {
        None => {

        }
        Some(group_id) => {
            let function = GroupFunctionService::select_function(&group_id).await;
            match function {
                None => {
                    add_group_function(&group_id).await;
                }
                Some(fun) => {
                    let string = fun.function_list.unwrap();
                    let result: Value = serde_json::from_str(string.as_str()).unwrap();
                    let function = result.as_object().unwrap();
                    let group = match &event {
                        Event::GroupMessageEvent(event) => {
                            Some(Group::new(event, bot))
                        }
                        _ => None
                    };
                    if function.get("groupHelp").unwrap().as_bool() == Some(true) {
                        group_change_handle(&event, bot).await;
                        if let Event::AddGroupRequest(event) = &event {
                            group_handle_module(&mut Request::new_add_group(event, bot)).await;
                        }
                    }
                    match group {
                        None => {}
                        Some(mut group) => {
                            open_group_function(&mut group).await;

                            if function.get("setu").unwrap().as_bool() == Some(true) {
                                setu_group_handle(&mut group).await;
                            }
                            if function.get("签到").unwrap().as_bool() == Some(true) {
                                sign_module_handle(&mut group).await;
                            }
                            if function.get("ai").unwrap().as_bool() == Some(true) {
                                ai_group_module_handle(&mut group).await;
                            }
                            if function.get("osusb").unwrap().as_bool() == Some(true) {
                                osu_sb_group_module_handle(&mut group).await;
                            }
                        }
                    }
                }
            }
        }
    }


    let friend = match &event {
        Event::FriendMessageEvent(event) => {
            Some(Friend::new(event, bot))
        }
        _ => None
    };

    match friend {
        None => {}
        Some(mut data) => {
            setu_friend_handle(&mut data).await;
        }
    }
    match &event {
        Event::AddFriendRequest(event) => {
            friend_handle_module(&mut Request::new_add_friend(event, bot)).await;
        }
        _ => {}
    }
}

pub async fn notice_event_handle(event: &Event, bot: &mut Bot) {
    match event {
        _ => {}
    };
}

pub fn meow_err<M: AsRef<str> + Display>(msg: M) -> String {
    format!("{}喵...", <M as Into<M>>::into(msg))
}

pub fn meow_ok<M: AsRef<str> + Display>(msg: M) -> String {
    format!("{}喵!", <M as Into<M>>::into(msg))
}

pub fn meow_warn<M: AsRef<str> + Display>(msg: M) -> String {
    format!("{}喵?", <M as Into<M>>::into(msg))
}

pub fn meow_log(msg: &str, r#type: i8) {
    match r#type {
        0 => info!("{}",meow_ok(msg)),
        1 => warn!("{}",meow_warn(msg)),
        2 => error!("{}",meow_err(msg)),
        _ => {}
    }
}

pub fn log_result(result: Option<ResultFrame>) {
    match result {
        None => {}
        Some(result) => {
            if result.ok {
                info!("[Bot] {} - {}",result.ok,result.message_id);
                return;
            } else {
                warn!("[Bot] {} - {}",result.ok,result.data.unwrap());
                return;
            }
        }
    }
}

pub fn log_result_by_return(result: Option<ResultFrame>) -> Option<ResultFrame> {
    let frame = match result {
        None => None,
        Some(result) => {
            return if result.ok {
                info!("[Bot] {} - {}",result.ok,result.message_id);
                Some(result)
            } else {
                warn!("[Bot] {} - {}",result.ok,result.data.unwrap());
                None
            };
        }
    };
    frame
}

pub fn bot_name_compound(num: i8, msg: &str) -> String {
    let name = CONTEXT.bot_config.bot_name.as_ref().unwrap();
    let id = CONTEXT.bot_config.bot_id.as_ref().unwrap();
    match num {
        0 => {
            format!("{}{}", name, msg)
        }
        1 => {
            format!("{}{}", msg, name)
        }
        _ => { format!("") }
    }
}
