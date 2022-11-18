use log::{error, info, warn};
use serde_json::Value;

use crate::core::bot::{Bot, Frame};
use crate::core::event::*;
use crate::core::friend::Friend;
use crate::core::group::Group;

use crate::domain::{GroupFunction, Setu};
use crate::handler::{group_function_handle, setu_friend_handle, setu_group_handle, sign_module_handle};
use crate::service::{CONTEXT, GroupFunctionService, SetuService};

pub async fn event_handle(event: Event, bot: &mut Bot) {
    let mut bot = bot.clone();
    let mut group = match &event {
        Event::GroupMessageEvent(event) => {
            info!("G::{} >Q::{} >{}",&event.group_id,&event.user_id,&event.raw_message);
            Some(Group::new(event, &mut bot))
        }
        _ => None
    };
    let mut friend = match &event {
        Event::FriendMessageEvent(event) => {
            info!("Q::{} >{}",&event.user_id,&event.raw_message);
            Some(Friend::new(event, &mut bot))
        }
        _ => None
    };

    // match &event {
    //     Event::GroupFileUpload(_) => {}
    //     Event::GroupAdminChange(_) => {}
    //     Event::GroupMemberDecrease(_) => {}
    //     Event::GroupMemberIncrease(_) => {}
    //     Event::GroupBan(_) => {}
    //     Event::FriendAdd(_) => {}
    //     Event::GroupMessageRecall(_) => {}
    //     Event::FriendMessageRecall(_) => {}
    //     Event::FriendPoke(_) => {}
    //     Event::GroupPoke(_) => {}
    //     Event::TipsForLuckyKingOfRedPackets(_) => {}
    //     Event::GroupMemberHonorChangePrompt(_) => {}
    //     Event::GroupMemberBusinessCardUpdate(_) => {}
    //     Event::OfflineFileReceived(_) => {}
    //     Event::AddFriendRequest(_) => {}
    //     Event::AddGroupRequest(_) => {}
    //     Event::OtherClientOnlineStatusChanges(_) => {}
    //     Event::ApiResult(_) => {}
    // }
    match group {
        None => {}
        Some(mut data) => {
            group_function_handle(&mut data).await;
            let function = GroupFunctionService::select_function(&data.group_id).await;
            match function {
                None => {}
                Some(fun) => {
                    let string = fun.function_list.unwrap();
                    let result:Value = serde_json::from_str(string.as_str()).unwrap();
                    let option = result.as_object().unwrap();
                    if option.get("setu").unwrap().as_bool() == Some(true){
                        setu_group_handle(&mut data).await;
                    }
                    if option.get("签到").unwrap().as_bool() == Some(true) {
                        sign_module_handle(&mut data).await;
                    }
                }
            }

        }
    }
    match friend {
        None => {}
        Some(mut data) => {
            setu_friend_handle(&mut data).await;
        }
    }
}


pub async fn meow_err(msg: &str) -> String {
    format!("{} 喵...", msg)
}

pub async fn meow_ok(msg: &str) -> String {
    format!("{} 喵!", msg)
}

pub async fn meow_warn(msg: &str) -> String {
    format!("{} 喵?", msg)
}

pub async fn meow_log(msg: &str, r#type: i8) {
    match r#type {
        0 => info!("{}",meow_ok(msg).await),
        1 => warn!("{}",meow_warn(msg).await),
        2 => error!("{}",meow_err(msg).await),
        _ => {}
    }
}

pub async fn handle_frame(frame: Option<Frame>) {
    match frame {
        None => {}
        Some(frame) => {
            if frame.ok {
                info!("[Bot] {} - {}",frame.ok,frame.message_id);
                return;
            } else {
                warn!("[Bot] {} - {}",frame.ok,frame.data.unwrap());
                return;
            }
        }
    }
}

pub async fn handle_frame_return(frame: Option<Frame>) -> Option<Frame> {
    let frame = match frame {
        None => None,
        Some(frame) => {
            return if frame.ok {
                info!("[Bot] {} - {}",frame.ok,frame.message_id);
                Some(frame.clone())
            } else {
                warn!("[Bot] {} - {}",frame.ok,frame.data.unwrap());
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