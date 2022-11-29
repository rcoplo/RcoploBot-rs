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
use crate::bot::*;
use crate::bot::bot_help::bot_help_group_handle;
use crate::service::{CONTEXT, GroupFunctionService, SetuService};
use crate::util::regex_utils::contain;

pub async fn event_handle(event: Event, bot: &mut Bot) {
    let group = match &event {
        Event::GroupMessageEvent(event) => {
            Some(Group::new(event, bot))
        }
        _ => None
    };
    let friend = match &event {
        Event::FriendMessageEvent(event) => {
            Some(Friend::new(event, bot))
        }
        _ => None
    };

    let group_id = get_group_id(&event);
    //获取group_id 方便管理群功能
    match group_id {
        None => {
        }
        Some(group_id) => {
            //查询当前群的功能
            let function = GroupFunctionService::select_function(&group_id).await;
            match function {
                None => {
                    //如果数据库没有数据则添加
                    add_group_function(&group_id).await;
                }
                Some(fun) => {
                    let string = fun.function_list.unwrap();
                    let result: Value = serde_json::from_str(string.as_str()).unwrap();
                    let function = result.as_object().unwrap();

                    if function.get("groupHelp").unwrap().as_bool() == Some(true) {
                        group_change_handle(&event, bot).await;
                        if let Event::AddGroupRequest(event) = &event {
                            group_handle_module(&mut Request::new_add_group(event, bot)).await;
                        }
                    }
                    match group {
                        None => {}
                        Some(mut group) => {
                            bot_help_group_handle(&mut group,function).await;
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
    match friend {
        None => {}
        Some(mut friend) => {
            setu_friend_handle(&mut friend).await;
        }
    }
    
    match &event {
        Event::FriendMessageEvent(event) => {

        }
        Event::AddFriendRequest(event) => {
            friend_handle_module(&mut Request::new_add_friend(event, bot)).await;
        }
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

fn get_group_id(event: &Event) -> Option<i64>{
    match &event {
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
    }

}