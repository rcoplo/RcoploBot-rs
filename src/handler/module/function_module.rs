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
use crate::handler::{log_result, meow_err, meow_ok, meow_warn};

use crate::pool;
use crate::service::{CONTEXT, GroupFunctionService};
use crate::util::regex_utils::{contain, contain_int};


pub async fn group_function_handle(group_id:&i64) {
    let function = GroupFunctionService::select_function( group_id).await;
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
                group_id: Some(group_id.clone()),
                function_list: Some(value.to_string()),
                modify_time: Some(FastDateTime::now())
            };
            GroupFunctionService::insert_function(function1).await;
        }
    }
}


pub async fn open_group_function(group: &mut Group){
    let vec1 = CONTEXT.bot_config.super_administrator.clone();
    if contain_int(&group.user_id,&vec1) {
        if contain(&group.message[0],&vec!["开启[\\s]+\\w+"]) {
            if GroupFunctionService::open_function(&group.group_id,&group.message_list[1]).await {
                let result = group.send_group_msg( vec![text(meow_ok(format!("开启 {} 功能成功",&group.message_list[1])))]).await;
                log_result(result);
            }else {
                let result = group.send_group_msg( vec![text(meow_warn(format!("开启 {} 功能失败,可能该功能并不存在",&group.message_list[1])))]).await;
                log_result(result);
            };

        }else if contain(&group.message[0],&vec!["关闭[\\s]+\\w+"])   {
            if GroupFunctionService::close_function(&group.group_id,&group.message_list[1]).await {
                let result = group.send_group_msg( vec![text(meow_ok(format!("关闭 {} 功能成功",&group.message_list[1])))]).await;
                log_result(result);
            } else {
                let result = group.send_group_msg( vec![text(meow_warn(format!("关闭 {} 功能失败,可能该功能并不存在",&group.message_list[1])))]).await;
                log_result(result);
            };
        }
    }


}
pub async fn friend_function_handle(event:&FriendMessageEvent, bot: &mut Bot) {


}