use std::collections::HashMap;
use std::thread;
use std::thread::Thread;

use log::info;
use regex::{Regex, Replacer};
use serde_yaml::mapping::Index;
use serde_yaml::Value;
use tokio::time;
use crate::core::bot::{Bot, ResultFrame};
use crate::core::Cq;
use crate::core::event::*;
use crate::core::friend::Friend;
use crate::core::group::Group;
use crate::core::message::*;
use crate::domain::Setu;
use crate::handler::api::{get_lolicon, get_lolicon_list, get_lolicon_list_tag, get_lolicon_tag};
use crate::handler::{log_result, log_result_by_return, meow_err, meow_log};
use crate::handler::bot_help::{BOT_HELP, BotHelp, Help};
use crate::service::{CONTEXT, GroupFunctionService, SetuService};
use crate::util::regex_utils::{contain};

pub struct SetuHelp;

impl BotHelp for SetuHelp {
    fn new() -> Help<'static> {
        Help {
            module_name: "色图".to_string(),
            module_name_abbreviation: "setu".to_string(),
            module_order: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("setu",vec!["/色图", "/瑟图"]),
                ("setu_tag",vec!["/色图[\\s+](.*)", "/瑟图[\\s+](.*)"]),
                ("setu_rand",vec!["/rand 色图", "/rand 瑟图"]),
                ("setu_list",vec![r"(\d)张色图"]),
                ("setu_list_tag",vec!["(\\d)张色图[\\s+](.*)"]),
            ])),
            module_default: false,
            module_help: vec![
                "指令: /色图",
                "参数: {tag}|{tag}",
                "------------------------",
                "指令: {num}张色图",
                "参数: {num}指1~20的数字",
                "     {tag}|{tag}",
                "     {tag}中间以英文 `|` 号间隔",
                "参数和指令中间需有空格",
            ],
        }
    }
}

pub async fn setu_friend_handle(friend: &mut Friend) {
    // let mut friend = friend.clone();
    let setu_help = &BOT_HELP.help.get("setu").unwrap().module_order;
    if contain(&friend.message, setu_help.get("setu").unwrap()) {
        meow_log("setu_friend", 0);
        setu_friend(friend).await;
    } else if contain(&friend.message, setu_help.get("setu_tag").unwrap()) {
        meow_log("setu_friend_tag", 0);
        setu_friend_tag(friend).await;
    } else if contain(&friend.message, setu_help.get("setu_rand").unwrap()) {
        meow_log("rand_setu_friend", 0);
        rand_setu_friend(friend).await;
    } else if contain(&friend.message, setu_help.get("setu_list").unwrap()) {
        meow_log("setu_friend_list", 0);
        setu_friend_list(friend).await;
    } else if contain(&friend.message, setu_help.get("setu_list_tag").unwrap()) {
        meow_log("setu_friend_list_tag", 0);
        setu_friend_list_tag(friend).await;
    };
}

pub async fn setu_group_handle(group: &mut Group) {
    let setu_help = &BOT_HELP.help.get("setu").unwrap().module_order;
    if contain(&group.message, setu_help.get("setu").unwrap()) {
        meow_log("setu_group", 0);
        setu_group(group).await;
    } else if contain(&group.message, setu_help.get("setu_tag").unwrap()) {
        meow_log("setu_group_tag", 0);
        setu_group_tag(group).await;
    } else if contain(&group.message, setu_help.get("setu_rand").unwrap()) {
        meow_log("rand_setu_group", 0);
        // rand_setu_group(&mut group).await;
    } else if contain(&group.message, setu_help.get("setu_list").unwrap()) {
        meow_log("setu_group_list", 0);
        setu_group_list(group).await;
    } else if contain(&group.message, setu_help.get("setu_list_tag").unwrap()) {
        meow_log("setu_group_list_tag", 0);
        setu_group_list_tag(group).await;
    };
}

async fn setu_friend(friend: &mut Friend) {
    let lolicon = get_lolicon().await;
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
            let result = friend.send_private_msg(vec).await;
            log_result(result);
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
                text(format!("Title: {}\n", &setu.title.unwrap())),
                text(format!("pid: {}\n", &setu.pid.unwrap())),
                image(&setu.urls.unwrap()),
            ];
            let result = friend.send_private_msg(vec).await;
            log_result(result);
        }
    }
}

async fn setu_friend_list(friend: &mut Friend) {
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(friend.message.as_str(), "$last").parse::<i64>().unwrap();

    let lolicon = get_lolicon_list(cow).await;
    match lolicon {
        None => {}
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", &s.title.unwrap())),
                    text(format!("pid: {}\n", &s.pid.unwrap())),
                    image(&s.urls.unwrap()),
                ];
                let result = friend.send_private_msg(vec).await;
                log_result(result);
            }
        }
    }
}

async fn setu_friend_list_tag(friend: &mut Friend) {
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(friend.message.as_str(), "$last").parse::<i64>().unwrap();
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
                let result = friend.send_private_msg(vec).await;
                log_result(result);
            }
        }
    }
}

async fn rand_setu_friend(friend: &mut Friend) {
    let setu = SetuService::rand_setu().await;
    match setu {
        None => {

        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", &setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", &setu.pid.unwrap()).as_str()),
                image(&setu.urls.unwrap().as_str()),
            ];
            let result = friend.send_private_msg(vec).await;
            log_result(result);
        }
    }
}

async fn setu_group(group: &mut Group) {
    let mut group = group.clone();
    let lolicon = get_lolicon().await;
    match lolicon {
        None => {
            let result = group.send_group_msg(vec![text(meow_err("色图获取失败乐"))]).await;
            log_result(result);
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", setu.pid.unwrap()).as_str()),
                image(setu.urls.unwrap().as_str()),
            ];
            let result = group.send_group_msg(vec).await;
            let result_return = log_result_by_return(result);
            match result_return {
                None => {
                    let result = group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                    log_result(result);
                }
                Some(result) => {
                    tokio::spawn( async move{
                        delete_msg(&mut group, result.message_id,&CONTEXT.config.setu.withdraw_time).await;
                    });
                }
            }
        }
    }
}

async fn setu_group_list(group: &mut Group) {
    let mut group = group.clone();
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(group.message.as_str(), "$last").parse::<i64>().unwrap();
    let lolicon = get_lolicon_list(cow).await;
    match lolicon {
        None => {
            let result = group.send_group_msg(vec![text(meow_err("色图获取失败乐"))]).await;
            log_result(result);
        }
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", s.pid.unwrap()).as_str()),
                    image(s.urls.unwrap().as_str()),
                ];
                let result = group.send_group_msg(vec).await;
                let result_return = log_result_by_return(result);
                match result_return {
                    None => {
                        let result = group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                        log_result(result);
                    }
                    Some(result) => {
                        let mut group = group.clone();
                        tokio::spawn(  async move{
                            delete_msg(&mut group, result.message_id,&CONTEXT.config.setu.withdraw_time).await;
                        });
                    }
                }
            }
        }
    }
}

async fn setu_group_list_tag(group: &mut Group) {
    let mut group = group.clone();

    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(group.message.as_str(), "$last").parse::<i64>().unwrap();
    let mut vec = Vec::new();
    let split: Vec<_> = group.message_list[1].split("|").collect();
    for str in split {
        vec.push(str.to_string())
    }

    let lolicon = get_lolicon_list_tag(cow, vec).await;
    match lolicon {
        None => {
            let result = group.send_group_msg(vec![text(meow_err("色图获取失败乐"))]).await;
            log_result(result);
        }
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", &s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", &s.pid.unwrap()).as_str()),
                    image(&s.urls.unwrap().as_str()),
                ];
                let result = group.send_group_msg(vec).await;
                let result_return = log_result_by_return(result);
                match result_return {
                    None => {
                        let result = group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                        log_result(result);
                    }
                    Some(result) => {
                        let mut group = group.clone();
                        tokio::spawn(async move{
                            delete_msg(&mut group, result.message_id,&CONTEXT.config.setu.withdraw_time).await;
                        });
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
            let result = group.send_group_msg(vec![text(meow_err("色图获取失败乐"))]).await;
            log_result(result);
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", &setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", &setu.pid.unwrap()).as_str()),
                image(&setu.urls.unwrap().as_str()),
            ];
            let result = group.send_group_msg(vec).await;
            let result_return = log_result_by_return(result);
            match result_return {
                None => {
                    let result = group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                    log_result(result);
                }
                Some(result) => {
                    tokio::spawn( async move{
                        delete_msg(&mut group, result.message_id,&CONTEXT.config.setu.withdraw_time).await;
                    });
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
            let result = group.send_group_msg(vec).await;
            let result_return = log_result_by_return(result);
            match result_return {
                None => {
                    let result = group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                    log_result(result);
                }
                Some(result) => {
                    tokio::spawn(  async move{
                        delete_msg(&mut group, result.message_id,&CONTEXT.config.setu.withdraw_time).await;
                    });
                }
            }
        }
    }
}


async fn delete_msg(group: &mut Group,message_id:i64, time: &u64) {
    time::sleep(time::Duration::from_secs(*time)).await;
    let result = group.delete_msg(message_id).await;
    match result {
        None => {}
        Some(result) => {
            info!("[Bot] {} - 消息撤回成功!",result.ok);
            return;
        }
    };
}