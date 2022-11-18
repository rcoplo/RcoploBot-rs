// use std::collections::HashMap;
// use std::fmt::format;
// use std::fs;
// use std::iter::Map;
// use log::info;
// use rand::Rng;
// use serde_json::Value;
// use crate::core::bot::Bot;
// use crate::core::event::{Event, GroupMessageEvent};
// use crate::util::regex_utils::contain;
// use crate::core::message::*;
// use crate::handler::{handle_frame};
// use crate::util::file::get_tmp_path;
//
// pub async fn group_recreational_module_handle(event:&Event,bot:&mut Bot){
//     let mut bot = bot.clone();
//     match event{
//         Event::GroupMessageEvent(event) => {
//             group_handle(&event, &mut bot).await
//         }
//         _ => {}
//     }
//
// }
//
// pub async fn group_handle(event:& GroupMessageEvent, bot: &mut Bot) {
//     let mut bot = bot.clone();
//     if contain(&event.raw_message, vec!["日群友"]) {
//         supermarket_group_member(&event, &mut bot).await;
//     };
//     if contain(&event.raw_message, vec!["日群主"]) {
//         let frame = bot.send_group_msg(event.group_id, vec![at(&event.user_id), text("绒布球不乖了噢")]).await;
//         handle_frame(frame).await;
//     };
// }
//
// async fn supermarket_group_member(event:&GroupMessageEvent, bot: &mut Bot){
//     let mut bot = bot.clone();
//     let path = format!("{}/{}.json", get_tmp_path(vec!["json"]), "supermarket_group_member");
//     let file = fs::read(path).unwrap();
//     let data = String::from_utf8(file).unwrap();
//     let json:Value = serde_json::from_str(&data).unwrap();
//     for json in json.as_array().unwrap() {
//         let json = json.as_object().unwrap();
//         let cp_1 = json["cp_1"].as_i64().unwrap();
//         if cp_1 == event.user_id {
//             let cp_2 = json["cp_2"].as_i64().unwrap();
//             let stranger = bot.get_stranger_info(cp_2).await.unwrap();
//             let msg = vec![
//                 at(&event.user_id),
//                 text(format!(" 你今天的cp是{}\n", stranger.nickname).as_str()),
//                 image(get_user_avatar(stranger.user_id).as_str()),
//             ];
//             let frame = bot.send_group_msg(event.group_id, msg).await;
//
//         } else {
//             let mut map = HashMap::new();
//             let vec1 = bot.get_group_member_list(event.group_id).await.unwrap();
//             let rand = rand::thread_rng().gen_range(0..vec1.len());
//             let member_info = vec1.get(rand).unwrap();
//             map.insert("cp_1".to_string(), Value::from(event.user_id));
//             map.insert("cp_2".to_string(), Value::from(member_info.user_id));
//             let string = serde_json::to_string(&map).unwrap();
//             info!("{}", string);
//         };
//     }
// }


