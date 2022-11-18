use std::collections::HashMap;
use log::{error, info};
use once_cell::sync::Lazy;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::rbdc::Error;
use serde_json::{json, Value};
use crate::core::bot::Bot;
use crate::core::event::{Event, FriendMessageEvent, GroupMessageEvent};
use crate::core::message::text;
use crate::domain::GroupFunction;
use crate::handler::bot_help::BOT_HELP;
use crate::handler::handle_frame;

use crate::pool;
use crate::service::GroupFunctionService;
use crate::util::regex_utils::contain;


pub async fn group_function_handle(event:&GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    let function = GroupFunctionService::select_function( &event.group_id).await;
    match function {
        Some(data) => {
        }
        None => {
            let map_help = &BOT_HELP.help;
            let mut map = HashMap::new();
            for (abb,help) in map_help.iter() {
                map.insert(abb.clone(),help.module_default);
            }
            let value = Value::from_iter(map);
            let function1 = GroupFunction {
                id: 0,
                group_id: Some(event.group_id.clone()),
                function_list: Some(value.to_string()),
                modify_time: Some(FastDateTime::now())
            };
            GroupFunctionService::insert_function(function1).await;
        }
    }
    open_group_function(&event,&mut bot).await;

}


pub async fn open_group_function(event:&GroupMessageEvent, bot: &mut Bot){
    if contain(&event.raw_message,vec!["开启[\\s]+\\w+"]) {
        info!("{}",&event.raw_message);
        let mut vec = vec![];
        for x in event.raw_message.split_whitespace() {
            vec.push(x);
        }
       if GroupFunctionService::open_function(&event.group_id,&vec[1]).await {
           let frame = bot.send_group_msg(event.group_id, vec![text("开启 "), text(vec[1]), text(" 功能成功！")]).await;
           handle_frame(frame).await;
       } ;

    }else if contain(&event.raw_message,vec!["关闭[\\s]+\\w+"])   {
        let mut vec = vec![];
        for x in event.raw_message.split_whitespace() {
            vec.push(x);
        }
        if GroupFunctionService::close_function(&event.group_id,&vec[1]).await {
            let frame = bot.send_group_msg(event.group_id, vec![text("关闭 "), text(vec[1]), text(" 功能成功！")]).await;
            handle_frame(frame).await;
        } ;
    }

}
pub async fn friend_function_handle(event:&FriendMessageEvent, bot: &mut Bot) {


}