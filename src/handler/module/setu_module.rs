use std::thread::Thread;

use log::info;
use tokio::time;
use crate::core::bot::{Bot, Frame};
use crate::core::Cq;
use crate::core::event::*;
use crate::core::message::*;
use crate::domain::Setu;
use crate::handler::api::get_lolicon;
use crate::service::{CONTEXT, SetuService};
use crate::util::regex_utils::contain;


pub async fn setu_module_handle(event: &Event, bot: &mut Bot) {
    let mut bot = bot.clone();
    match event {
        Event::FriendMessageEvent(event) => {
            friend_handle(&event, &mut bot).await
        }
        Event::GroupMessageEvent(event) => {
            group_handle(&event, &mut bot).await
        }
        _ => {}
    }
}

pub async fn friend_handle(event: &FriendMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    if contain(event.raw_message.as_str(), vec!["/色图", "/瑟图"]) {
        get_setu_friend(&event, &mut bot).await;
    };
    if contain(event.raw_message.as_str(), vec!["/rand 色图", "/rand 瑟图"]) {
        rand_setu_friend(&event, &mut bot).await;
    };
}

pub async fn group_handle(event:& GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    if contain(event.raw_message.as_str(), vec!["/色图", "/瑟图"]) {
        get_setu_group(&event, &mut bot).await;
    };
    if contain(event.raw_message.as_str(), vec!["/rand 色图", "/rand 瑟图"]) {
        rand_setu_group(&event, &mut bot).await;
    };
}

async fn get_setu_friend(event: &FriendMessageEvent, bot: &mut Bot) {
    let lolicon = get_lolicon().await;
    match lolicon {
        None => {}
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap())),
                text(format!("pid: {}\n", setu.pid.unwrap())),
                image(setu.urls.unwrap()),
            ];
            let frame = bot.send_private_msg(event.user_id, vec).await;
            match frame {
                None => {}
                Some(frame) => {
                    info!("[Bot] {} - {}",frame.ok,frame.message_id);
                }
            }
        }
    }
}

async fn rand_setu_friend(event:& FriendMessageEvent, bot: &mut Bot) {
    let setu = SetuService::rand_setu().await;
    match setu {
        None => {}
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap())),
                text(format!("pid: {}\n", setu.pid.unwrap())),
                image(setu.urls.unwrap()),
            ];
            let frame = bot.send_private_msg(event.user_id, vec).await;
            match frame {
                None => {}
                Some(frame) => {
                    info!("[Bot] {} - {}",frame.ok,frame.message_id);
                }
            }
        }
    }
}

async fn get_setu_group(event: &GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let lolicon = get_lolicon().await;
    match lolicon {
        None => {}
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap())),
                text(format!("pid: {}\n", setu.pid.unwrap())),
                image(setu.urls.unwrap()),
            ];
            let frame = bot.send_group_msg(event.group_id, vec).await;
            match frame {
                None => {}
                Some(frame) => {
                    if frame.ok{
                        info!("[Bot] {} - {}",frame.ok,frame.message_id);
                        delete_msg(&mut bot, frame.message_id, 60).await;
                    }
                    let frame = bot.send_group_msg(event.group_id, vec![text("这张色图太😍了,我自己看看就好了~".to_string())]).await;
                    match frame {
                        None => {}
                        Some(frame) => {
                            if frame.ok{
                                info!("[Bot] {} - {}",frame.ok,frame.message_id);
                                delete_msg(&mut bot, frame.message_id, 60).await;
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn rand_setu_group(event:& GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let setu = SetuService::rand_setu().await;
    match setu {
        None => {}
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap())),
                text(format!("pid: {}\n", setu.pid.unwrap())),
                image(setu.urls.unwrap()),
            ];
            let frame = bot.send_group_msg(event.group_id, vec).await;
            match frame {
                None => {}
                Some(frame) => {
                    if frame.ok{
                        info!("[Bot] {} - {}",frame.ok,frame.message_id);
                        delete_msg(&mut bot, frame.message_id, 60).await;
                    }
                    let frame = bot.send_group_msg(event.group_id, vec![text("这张色图太😍了,我自己看看就好了~".to_string())]).await;
                    match frame {
                        None => {}
                        Some(frame) => {
                            if frame.ok{
                                info!("[Bot] {} - {}",frame.ok,frame.message_id);
                                delete_msg(&mut bot, frame.message_id, 60).await;
                            }
                        }
                    }
                }
            }
        }
    }
}


async fn delete_msg(bot: &mut Bot, msg_id: i64, time: u64) {
    let mut bot = bot.clone();
    time::sleep(time::Duration::from_secs(time)).await;
    let handle = tokio::spawn(async move {
        let frame = bot.delete_msg(msg_id).await;
        match frame {
            None => {}
            Some(frame) => {
                info!("[Bot] {} - 消息撤回成功!",frame.ok);
                return;
            }
        };
    });
    handle.abort();
}