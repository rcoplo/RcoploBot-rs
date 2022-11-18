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
use crate::core::friend::Friend;
use crate::core::group::Group;
use crate::core::message::*;
use crate::domain::Setu;
use crate::handler::api::{get_lolicon, get_lolicon_list, get_lolicon_list_tag, get_lolicon_tag};
use crate::handler::{handle_frame, handle_frame_return, meow_log};
use crate::handler::bot_help::{BotHelp, Help};
use crate::service::{CONTEXT, GroupFunctionService, SetuService};
use crate::util::regex_utils::{contain, replace_regex};

pub struct SetuHelp;

impl BotHelp for SetuHelp {
    fn new() -> Help {
        Help {
            module_name: "色图".to_string(),
            module_name_abbreviation: "setu".to_string(),
            module_default: false,
            module_help: vec![
                "指令: /色图".to_string(),
                "参数: [tag]|[tag]".to_string(),
                "------------------------".to_string(),
                "指令: [num]张色图".to_string(),
                "参数: [num]指1~20的数字".to_string(),
                "     [tag]|[tag]".to_string(),
                "     [tag]中间以英文 `|` 号间隔".to_string(),
                "参数和指令中间需有空格".to_string(),
            ],
        }
    }
}

pub async fn setu_friend_handle(friend: &mut Friend) {
    let mut friend = friend.clone();
    if contain(&friend.raw_message, vec!["/色图", "/瑟图"]) {
        meow_log("setu_friend", 0).await;
        setu_friend(&mut friend).await;
    } else if contain(&friend.raw_message, vec!["/色图[\\s+](.*)", "/瑟图[\\s+](.*)"]) {
        meow_log("setu_friend_tag", 0).await;
        setu_friend_tag(&mut friend).await;
    } else if contain(&friend.raw_message, vec!["/rand 色图", "/rand 瑟图"]) {
        meow_log("rand_setu_friend", 0).await;
        rand_setu_friend(&mut friend).await;
    } else if contain(&friend.raw_message, vec![r"(\d)张色图"]) {
        meow_log("setu_friend_list", 0).await;
        setu_friend_list(&mut friend).await;
    } else if contain(&friend.raw_message, vec!["(\\d)张色图[\\s+](.*)"]) {
        meow_log("setu_friend_list_tag", 0).await;
        setu_friend_list_tag(&mut friend).await;
    };
}

pub async fn setu_group_handle(group: &mut Group) {
    let mut group = group.clone();
    if contain(&group.raw_message, vec!["/色图", "/瑟图"]) {
        meow_log("setu_group", 0).await;
        setu_group(&mut group).await;
    } else if contain(&group.raw_message, vec!["/色图[\\s+](.*)", "/瑟图[\\s+](.*)"]) {
        meow_log("setu_group_tag", 0).await;
        setu_group_tag(&mut group).await;
    } else if contain(&group.raw_message, vec!["/rand 色图", "/rand 瑟图"]) {
        meow_log("rand_setu_group", 0).await;
        // rand_setu_group(&mut group).await;
    } else if contain(&group.raw_message, vec![r"(\d)张色图"]) {
        meow_log("setu_group_list", 0).await;
        setu_group_list(&mut group).await;
    } else if contain(&group.raw_message, vec!["(\\d)张色图[\\s+](.*)"]) {
        meow_log("setu_group_list_tag", 0).await;
        setu_group_list_tag(&mut group).await;
    };
}

async fn setu_friend(friend: &mut Friend) {
    let lolicon = get_lolicon().await;
    match lolicon {
        None => {
            // rand_setu_friend(&event, &mut bot).await;
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", &setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", &setu.pid.unwrap()).as_str()),
                image(&setu.urls.unwrap().as_str()),
            ];
            let frame = friend.send_private_msg(vec).await;
            handle_frame(frame).await;
        }
    }
}

async fn setu_friend_tag(friend: &mut Friend) {
    let mut vec = Vec::new();
    let split: Vec<_> = friend.message_list[1].split("|").collect();
    for str in split {
        vec.push(str.to_string())
    }
    let lolicon = get_lolicon_tag(vec).await;
    match lolicon {
        None => {
            rand_setu_friend(friend).await;
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", &setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", &setu.pid.unwrap()).as_str()),
                image(&setu.urls.unwrap().as_str()),
            ];
            let frame = friend.send_private_msg(vec).await;
            handle_frame(frame).await;
        }
    }
}

async fn setu_friend_list(friend: &mut Friend) {
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(friend.raw_message.as_str(), "$last").parse::<i64>().unwrap();

    let lolicon = get_lolicon_list(cow).await;
    match lolicon {
        None => {}
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", &s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", &s.pid.unwrap()).as_str()),
                    image(&s.urls.unwrap().as_str()),
                ];
                let frame = friend.send_private_msg(vec).await;
                handle_frame(frame).await;
            }
        }
    }
}

async fn setu_friend_list_tag(friend: &mut Friend) {
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(friend.raw_message.as_str(), "$last").parse::<i64>().unwrap();
    let mut vec = Vec::new();
    let split: Vec<_> = friend.message_list[1].split("|").collect();
    for str in split {
        vec.push(str.to_string())
    }
    let lolicon = get_lolicon_list_tag(cow, vec).await;
    match lolicon {
        None => {}
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", &s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", &s.pid.unwrap()).as_str()),
                    image(&s.urls.unwrap().as_str()),
                ];
                let frame = friend.send_private_msg(vec).await;
                handle_frame(frame).await;
            }
        }
    }
}

async fn rand_setu_friend(friend: &mut Friend) {
    let setu = SetuService::rand_setu().await;
    match setu {
        None => {}
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", &setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", &setu.pid.unwrap()).as_str()),
                image(&setu.urls.unwrap().as_str()),
            ];
            let frame = friend.send_private_msg(vec).await;
            handle_frame(frame).await;
        }
    }
}

async fn setu_group(group: &mut Group) {
    let mut group = group.clone();
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
            let frame = group.send_group_msg(vec).await;
            let frame_return = handle_frame_return(frame).await;
            match frame_return {
                None => {
                    let frame = group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                    handle_frame(frame).await;
                }
                Some(frame) => {
                    delete_msg(&mut group, &CONTEXT.config.setu.withdraw_time).await;
                }
            }
        }
    }
}

async fn setu_group_list(group: &mut Group) {
    let mut group = group.clone();
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(group.raw_message.as_str(), "$last").parse::<i64>().unwrap();
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
                let frame = group.send_group_msg(vec).await;
                let frame_return = handle_frame_return(frame).await;
                match frame_return {
                    None => {
                        let frame = group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                        handle_frame(frame).await;
                    }
                    Some(frame) => {
                        delete_msg(&mut group, &CONTEXT.config.setu.withdraw_time).await;
                    }
                }
            }
        }
    }
}

async fn setu_group_list_tag(group: &mut Group) {
    let mut group = group.clone();

    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(group.raw_message.as_str(), "$last").parse::<i64>().unwrap();
    let mut vec = Vec::new();
    let split: Vec<_> = group.message_list[1].split("|").collect();
    for str in split {
        vec.push(str.to_string())
    }

    let lolicon = get_lolicon_list_tag(cow, vec).await;
    match lolicon {
        None => {}
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", &s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", &s.pid.unwrap()).as_str()),
                    image(&s.urls.unwrap().as_str()),
                ];
                let frame = group.send_group_msg(vec).await;
                let frame_return = handle_frame_return(frame).await;
                match frame_return {
                    None => {
                        let frame = group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                        handle_frame(frame).await;
                    }
                    Some(frame) => {
                        delete_msg(&mut group, &CONTEXT.config.setu.withdraw_time).await;
                    }
                }
            }
        }
    }
}

async fn setu_group_tag(group: &mut Group) {
    let mut group = group.clone();
    let mut vec = Vec::new();
    let split: Vec<_> = group.message_list[1].split("|").collect();
    for str in split {
        vec.push(str.to_string())
    }
    let lolicon = get_lolicon_tag(vec).await;
    match lolicon {
        None => {
            // rand_setu_group(&event, &mut bot).await;
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", &setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", &setu.pid.unwrap()).as_str()),
                image(&setu.urls.unwrap().as_str()),
            ];
            let frame = group.send_group_msg(vec).await;
            let frame_return = handle_frame_return(frame).await;
            match frame_return {
                None => {
                    let frame = group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                    handle_frame(frame).await;
                }
                Some(frame) => {
                    delete_msg(&mut group, &CONTEXT.config.setu.withdraw_time).await;
                }
            }
        }
    }
}

async fn rand_setu_group(group: &mut Group) {
    let mut group = group.clone();
    let setu = SetuService::rand_setu().await;
    match setu {
        None => {}
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", setu.pid.unwrap()).as_str()),
                image(setu.urls.unwrap().as_str()),
            ];
            let frame = group.send_group_msg(vec).await;
            let frame_return = handle_frame_return(frame).await;
            match frame_return {
                None => {
                    let frame = group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                    handle_frame(frame).await;
                }
                Some(frame) => {
                    delete_msg(&mut group, &CONTEXT.config.setu.withdraw_time).await;
                }
            }
        }
    }
}


async fn delete_msg(group: &mut Group, time: &u64) {
    let mut group = group.clone();
    let time = time.clone();
    thread::spawn(move || async move {
        time::sleep(time::Duration::from_secs(time.clone())).await;
        let frame = group.delete_msg().await;
        match frame {
            None => {}
            Some(frame) => {
                info!("[Bot] {} - 消息撤回成功!",frame.ok);
                return;
            }
        };
    });
}