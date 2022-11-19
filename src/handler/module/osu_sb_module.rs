use crate::core::bot::Bot;
use crate::core::event::Event;


pub async fn osu_sb_module(event: &Event, bot: &mut Bot) {
    let mut bot = bot.clone();
    match event {
        Event::FriendMessageEvent(event) => {

        }
        Event::GroupMessageEvent(event) => {

        }
        _ => {}
    }
}