use log::info;

use crate::core::bot::Bot;
use crate::core::event::*;
use crate::domain::Setu;
use crate::handler::{group_recreational_module_handle, setu_module_handle};
use crate::service::{ SetuService};

pub async fn event_handle(event:Event, bot:&mut Bot) {
    let mut bot = bot.clone();
    setu_module_handle(&event, &mut bot).await;
    group_recreational_module_handle(&event, &mut bot).await;


}


pub async fn get_user_avatar(user_id:i64) -> String{
    format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=0",user_id )
}

pub async fn get_group_avatar(group_id:i64) -> String{
    format!("https://p.qlogo.cn/gh/{0}/{0}/0/",group_id)
}