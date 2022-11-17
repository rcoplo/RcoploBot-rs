use std::thread;
use std::thread::Thread;

use log::info;
use regex::{Regex, Replacer};
use serde_yaml::mapping::Index;
use serde_yaml::Value;
use tokio::time;
use crate::core::bot::{Bot, Frame};
use crate::core::Cq;
use crate::core::event::*;
use crate::core::message::*;
use crate::domain::Setu;
use crate::handler::api::{get_lolicon, get_lolicon_list, get_lolicon_list_tag, get_lolicon_tag};
use crate::handler::{handle_frame, handle_frame_return, meow_log};
use crate::service::{CONTEXT, SetuService};
use crate::util::regex_utils::{contain, replace_regex};


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
    if contain(&event.raw_message, vec!["/色图", "/瑟图"]) {
        meow_log("setu_friend", 0).await;
        setu_friend(&event, &mut bot).await;
    }else if contain(&event.raw_message, vec!["/色图[\\s+](.*)", "/瑟图[\\s+](.*)"]) {
        meow_log("setu_friend_tag", 0).await;
        setu_friend_tag(&event, &mut bot).await;
    }else if contain(&event.raw_message, vec!["/rand 色图", "/rand 瑟图"]) {
        meow_log("rand_setu_friend", 0).await;
        rand_setu_friend(&event, &mut bot).await;
    }else if contain(&event.raw_message, vec![r"(\d)张色图"]) {
        meow_log("setu_friend_list", 0).await;
        setu_friend_list(&event, &mut bot).await;
    }else if contain(&event.raw_message, vec!["(\\d)张色图[\\s+](.*)"]) {
        meow_log("setu_friend_list_tag", 0).await;
        setu_friend_list_tag(&event, &mut bot).await;
    };
}

pub async fn group_handle(event: &GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    if contain(&event.raw_message, vec!["/色图", "/瑟图"]) {
        meow_log("setu_group", 0).await;
        setu_group(&event, &mut bot).await;
    }else if contain(&event.raw_message, vec!["/色图[\\s+](.*)", "/瑟图[\\s+](.*)"]) {
        meow_log("setu_group_tag", 0).await;
        setu_group_tag(&event, &mut bot).await;
    }else if contain(&event.raw_message, vec!["/rand 色图", "/rand 瑟图"]) {
        meow_log("rand_setu_group", 0).await;
        // rand_setu_group(&event, &mut bot).await;
    }else if contain(&event.raw_message, vec![r"(\d)张色图"]) {
        meow_log("setu_group_list", 0).await;
        setu_group_list(&event, &mut bot).await;

    }else if contain(&event.raw_message, vec!["(\\d)张色图[\\s+](.*)"]) {
        meow_log("setu_group_list_tag", 0).await;
        setu_group_list_tag(&event, &mut bot).await;

    };
}

async fn setu_friend(event: &FriendMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let lolicon = get_lolicon().await;
    match lolicon {
        None => {
            rand_setu_friend(&event, &mut bot).await;
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", setu.pid.unwrap()).as_str()),
                image(setu.urls.unwrap().as_str()),
            ];
            let frame = bot.send_private_msg(event.user_id, vec).await;
            handle_frame(frame).await;
        }
    }
}

async fn setu_friend_tag(event: &FriendMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let mut vec = Vec::new();
    let mut tag = event.raw_message.split_whitespace().clone();
    for tag in tag {
        let string = tag.replace("/色图", "");
        if !string.is_empty() {
            for str in string.split("|") {
                vec.push(str.to_string())
            }
        }

    }
    let lolicon = get_lolicon_tag(vec).await;
    match lolicon {
        None => {
            rand_setu_friend(&event, &mut bot).await;
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", setu.pid.unwrap()).as_str()),
                image(setu.urls.unwrap().as_str()),
            ];
            let frame = bot.send_private_msg(event.user_id, vec).await;
            handle_frame(frame).await;
        }
    }
}

async fn setu_friend_list(event: &FriendMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let event = event;
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(event.raw_message.as_str(), "$last").parse::<i64>().unwrap();

    let lolicon = get_lolicon_list(cow).await;
    match lolicon {
        None => {}
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", s.pid.unwrap()).as_str()),
                    image(s.urls.unwrap().as_str()),
                ];
                let frame = bot.send_private_msg(event.user_id, vec).await;
                handle_frame(frame).await;
            }
        }
    }
}
async fn setu_friend_list_tag(event: &FriendMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let event = event;
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(event.raw_message.as_str(), "$last").parse::<i64>().unwrap();
    let mut vec = Vec::new();
    let mut tag = event.raw_message.split_whitespace().clone();
    for tag in tag {
        let string = tag.replace("/色图", "");
        if !string.is_empty() {
            for str in string.split("|") {
                vec.push(str.to_string())
            }
        }
    }
    let lolicon = get_lolicon_list_tag(cow,vec).await;
    match lolicon {
        None => {}
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", s.pid.unwrap()).as_str()),
                    image(s.urls.unwrap().as_str()),
                ];
                let frame = bot.send_private_msg(event.user_id, vec).await;
                handle_frame(frame).await;
            }
        }
    }
}

async fn rand_setu_friend(event: &FriendMessageEvent, bot: &mut Bot) {
    let setu = SetuService::rand_setu().await;
    match setu {
        None => {}
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", setu.pid.unwrap()).as_str()),
                image(setu.urls.unwrap().as_str()),
            ];
            let frame = bot.send_private_msg(event.user_id, vec).await;
            handle_frame(frame).await;
        }
    }
}

async fn setu_group(event: &GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let lolicon = get_lolicon().await;
    match lolicon {
        None => {
            // rand_setu_group(&event, &mut bot).await;
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", setu.pid.unwrap()).as_str()),
                image(setu.urls.unwrap().as_str()),
            ];
            let frame = bot.send_group_msg(event.group_id, vec).await;
            let frame_return = handle_frame_return(frame).await;
            match frame_return {
                None => {
                    let frame = bot.send_group_msg(event.group_id, vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                    handle_frame(frame).await;
                }
                Some(frame) => {
                    delete_msg(&mut bot, frame.message_id, 60).await;
                }
            }
        }
    }
}

async fn setu_group_list(event: &GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let event = event;
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(event.raw_message.as_str(), "$last").parse::<i64>().unwrap();

    let lolicon = get_lolicon_list(cow).await;
    match lolicon {
        None => {}
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", s.pid.unwrap()).as_str()),
                    image(s.urls.unwrap().as_str()),
                ];
                let frame = bot.send_group_msg(event.group_id, vec).await;
                let frame_return = handle_frame_return(frame).await;
                match frame_return {
                    None => {
                        let frame = bot.send_group_msg(event.group_id, vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                        handle_frame(frame).await;
                    }
                    Some(frame) => {
                        delete_msg(&mut bot, frame.message_id, 60).await;
                    }
                }
            }
        }
    }
}
async fn setu_group_list_tag(event: &GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let event = event;
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(event.raw_message.as_str(), "$last").parse::<i64>().unwrap();
    let mut vec = Vec::new();
    let mut tag = event.raw_message.split_whitespace().clone();
    for tag in tag {
        let string = tag.replace("/色图", "");
        if !string.is_empty() {
            for str in string.split("|") {
                vec.push(str.to_string())
            }
        }

    }
    let lolicon = get_lolicon_list_tag(cow,vec).await;
    match lolicon {
        None => {

        }
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", s.pid.unwrap()).as_str()),
                    image(s.urls.unwrap().as_str()),
                ];
                let frame = bot.send_group_msg(event.group_id, vec).await;
                let frame_return = handle_frame_return(frame).await;
                match frame_return {
                    None => {
                        let frame = bot.send_group_msg(event.group_id, vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                        handle_frame(frame).await;
                    }
                    Some(frame) => {
                        delete_msg(&mut bot, frame.message_id, 60).await;
                    }
                }
            }
        }
    }
}
async fn setu_group_tag(event: &GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let mut vec = Vec::new();
    let mut tag = event.raw_message.split_whitespace().clone();
    for tag in tag {
        let string = tag.replace("/色图", "");
        if !string.is_empty() {
            for str in string.split("|") {
                vec.push(str.to_string())
            }
        }
    }
    let lolicon = get_lolicon_tag(vec).await;
    match lolicon {
        None => {
            // rand_setu_group(&event, &mut bot).await;
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", setu.pid.unwrap()).as_str()),
                image(setu.urls.unwrap().as_str()),
            ];
            let frame = bot.send_group_msg(event.user_id, vec).await;
            handle_frame(frame).await;
        }
    }
}

async fn rand_setu_group(event: &GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let setu = SetuService::rand_setu().await;
    match setu {
        None => {}
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", setu.pid.unwrap()).as_str()),
                image(setu.urls.unwrap().as_str()),
            ];
            let frame = bot.send_group_msg(event.group_id, vec).await;
            let frame_return = handle_frame_return(frame).await;
            match frame_return {
                None => {
                    let frame = bot.send_group_msg(event.group_id, vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                    handle_frame(frame).await;
                }
                Some(frame) => {
                    delete_msg(&mut bot, frame.message_id, 60).await;
                }
            }
        }
    }
}


async fn delete_msg(bot: &mut Bot, msg_id: i64, time: u64) {
    let mut bot = bot.clone();
    thread::spawn( move|| async move {
        let mut bot = bot.clone();
        time::sleep(time::Duration::from_secs(time)).await;
        let frame = bot.delete_msg(msg_id).await;
        match frame {
            None => {}
            Some(frame) => {
                info!("[Bot] {} - 消息撤回成功!",frame.ok);
                return;
            }
        };
    });
}