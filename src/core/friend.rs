use crate::core::bot::{Bot, Frame};
use crate::core::event::{Event, FriendMessageEvent, FriendSender};
use crate::core::message::Message;

#[derive(Debug,Clone)]
pub struct Friend{
    pub user_id:i64,
    pub message_id:i64,
    pub raw_message:String,
    pub message_list:Vec<String>,
    pub sender:FriendSender,
    pub bot:Bot,
}

impl Friend {
    pub fn new(event: &FriendMessageEvent, bot:&mut Bot) -> Self{
        let mut vec = vec![];
        let binding = event.raw_message.clone();
        let msg_list:Vec<_> = binding.split_whitespace().collect();
        for msg in msg_list {
            vec.push(msg.to_string());
        }
        Self {
            user_id: event.user_id.clone(),
            message_id: event.message_id.clone(),
            raw_message: event.raw_message.clone(),
            message_list: vec,
            sender: event.sender.clone(),
            bot: bot.clone(),
        }
    }

    pub async fn send_private_msg(
        &mut self,
        message: Vec<Message>,
    ) -> Option<Frame> {
        self.bot.send_private_msg(self.user_id,message).await
    }

    pub async fn send_private_msg_cq(&mut self, user_id: i64, message: String) -> Option<Frame> {
        self.bot.send_private_msg_cq(self.user_id,message).await
    }

    pub async fn delete_msg(&mut self) -> Option<Frame> {
        self.bot.delete_msg(self.message_id).await
    }

    pub fn get_user_avatar(&self) -> String {
        format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=0", self.user_id)
    }

}