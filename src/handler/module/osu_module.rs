use crate::core::bot::Bot;
use crate::core::event::Event;


pub async fn osu_group_module_handle(event: &Event, bot: &mut Bot) {
    let mut bot = bot.clone();
    match event {
        Event::FriendMessageEvent(event) => {

        }
        Event::GroupMessageEvent(event) => {

        }
        _ => {}
    }
}