
use crate::core::bot::Bot;
use crate::core::event::Event;


pub async fn question_answer_module(event: &Event, bot: &mut Bot) {
    let mut bot = bot.clone();
    match event {
        Event::FriendMessageEvent(event) => {

        }
        Event::GroupMessageEvent(event) => {

        }
        _ => {}
    }
}