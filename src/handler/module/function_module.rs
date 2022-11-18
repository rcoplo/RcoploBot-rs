use std::collections::HashMap;
use log::{error, info};
use once_cell::sync::Lazy;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::rbdc::Error;
use serde_json::{json, Value};
use crate::core::bot::Bot;
use crate::core::event::{Event, FriendMessageEvent, GroupMessageEvent};
use crate::core::group::Group;
use crate::core::message::text;
use crate::domain::GroupFunction;
use crate::handler::bot_help::BOT_HELP;
use crate::handler::handle_frame;

use crate::pool;
use crate::service::GroupFunctionService;
use crate::util::regex_utils::contain;


pub async fn group_function_handle(group: &mut Group) {
    let mut group = group.clone();
    let function = GroupFunctionService::select_function( &group.group_id).await;
    match function {
        Some(_) => {}
        None => {
            let map_help = &BOT_HELP.help;
            let mut map = HashMap::new();
            for (abb,help) in map_help.iter() {
                map.insert(abb.clone(),help.module_default);
            }
            let value = Value::from_iter(map);
            let function1 = GroupFunction {
                id: 0,
                group_id: Some(group.group_id.clone()),
                function_list: Some(value.to_string()),
                modify_time: Some(FastDateTime::now())
            };
            GroupFunctionService::insert_function(function1).await;
        }
    }
    open_group_function(&mut group).await;

}


pub async fn open_group_function(group: &mut Group){
    if contain(&group.raw_message,vec!["开启[\\s]+\\w+"]) {

       if GroupFunctionService::open_function(&group.group_id,&group.message_list[1]).await {
           let frame = group.send_group_msg( vec![text("开启 "), text(group.message_list[1].as_str()), text(" 功能成功！")]).await;
           handle_frame(frame).await;
       } ;

    }else if contain(&group.raw_message,vec!["关闭[\\s]+\\w+"])   {

        if GroupFunctionService::close_function(&group.group_id,&group.message_list[1]).await {
            let frame = group.send_group_msg( vec![text("关闭 "), text(group.message_list[1].as_str()), text(" 功能成功！")]).await;
            handle_frame(frame).await;
        } ;
    }

}
pub async fn friend_function_handle(event:&FriendMessageEvent, bot: &mut Bot) {


}