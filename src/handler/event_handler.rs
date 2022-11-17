use log::{error, info, warn};

use crate::core::bot::{Bot, Frame};
use crate::core::event::*;
use crate::domain::Setu;
use crate::handler::{group_recreational_module_handle, setu_module_handle};
use crate::service::{ SetuService};

pub async fn event_handle(event:Event, bot:&mut Bot) {
    let mut bot = bot.clone();

    match &event {
        Event::FriendMessageEvent(event) => {
            info!("Q::{} >{}",&event.user_id,&event.raw_message);
        }
        Event::GroupMessageEvent(event) => {
            info!("G::{} >Q::{} >{}",&event.group_id,&event.user_id,&event.raw_message);
        }
        Event::GroupFileUpload(_) => {}
        Event::GroupAdminChange(_) => {}
        Event::GroupMemberDecrease(_) => {}
        Event::GroupMemberIncrease(_) => {}
        Event::GroupBan(_) => {}
        Event::FriendAdd(_) => {}
        Event::GroupMessageRecall(_) => {}
        Event::FriendMessageRecall(_) => {}
        Event::FriendPoke(_) => {}
        Event::GroupPoke(_) => {}
        Event::TipsForLuckyKingOfRedPackets(_) => {}
        Event::GroupMemberHonorChangePrompt(_) => {}
        Event::GroupMemberBusinessCardUpdate(_) => {}
        Event::OfflineFileReceived(_) => {}
        Event::AddFriendRequest(_) => {}
        Event::AddGroupRequest(_) => {}
        Event::OtherClientOnlineStatusChanges(_) => {}
        Event::ApiResult(_) => {}
    }

    setu_module_handle(&event, &mut bot).await;
    group_recreational_module_handle(&event, &mut bot).await;

}


pub async fn get_user_avatar(user_id:i64) -> String{
    format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=0",user_id )
}

pub async fn get_group_avatar(group_id:i64) -> String {
    format!("https://p.qlogo.cn/gh/{0}/{0}/0/",group_id)
}

pub async fn meow_err(msg:&str) -> String{
    format!("{} 喵...",msg)
}

pub async fn meow_ok(msg:&str) -> String{
    format!("{} 喵!",msg)
}
pub async fn meow_warn(msg:&str) -> String{
    format!("{} 喵?",msg)
}
pub async fn meow_log(msg:&str,r#type:i8){
    match r#type {
        0 => info!("{}",meow_ok(msg).await),
        1 => warn!("{}",meow_warn(msg).await),
        2 => error!("{}",meow_err(msg).await),
        _ => {}
    }
}
pub async fn handle_frame(frame:Option<Frame>){
    match frame {
        None => {}
        Some(frame) => {
            if frame.ok{
                info!("[Bot] {} - {}",frame.ok,frame.message_id);
                return;
            }else {
                warn!("[Bot] {} - {}",frame.ok,frame.data.unwrap());
                return;
            }
        }
    }
}
pub async fn handle_frame_return(frame:Option<Frame>) -> Option<Frame>{
   let frame = match frame {
        None => None,
        Some(frame) => {
            return if frame.ok {
                info!("[Bot] {} - {}",frame.ok,frame.message_id);
                Some(frame.clone())
            } else {
                warn!("[Bot] {} - {}",frame.ok,frame.data.unwrap());
                None
            }
        }
    };
    frame
}

