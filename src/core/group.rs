use crate::core::bot::{Bot, Frame};
use crate::core::event::{Event, GroupMessageEvent, GroupSender};
use crate::core::message::Message;

#[derive(Debug,Clone)]
pub struct Group {
    pub group_id: i64,
    pub user_id: i64,
    pub message_id: i64,
    pub raw_message:String,
    pub message_list:Vec<String>,
    pub sender: GroupSender,
    pub bot: Bot,
}

impl Group {
    pub fn new(event: &GroupMessageEvent, bot:&mut Bot) -> Self {
        let mut vec = vec![];
        let binding = event.raw_message.clone();
        let msg_list:Vec<_> = binding.split_whitespace().collect();
        for msg in msg_list {
            vec.push(msg.to_string());
        }
        Self {
            group_id: event.group_id.clone(),
            user_id:event.user_id.clone(),
            message_id: event.message_id.clone(),
            raw_message:event.raw_message.clone(),
            message_list: vec,
            sender: event.sender.clone(),
            bot: bot.clone(),
        }
    }
    pub async fn send_group_msg(&mut self, message: Vec<Message>) -> Option<Frame> {
        self.bot.send_group_msg(self.group_id, message).await
    }

    pub async fn send_group_msg_cq(&mut self, message: String) -> Option<Frame> {
        self.bot.send_group_msg_cq(self.group_id, message).await
    }

    pub async fn send_group_forward_msg(&mut self, message: Vec<Message>) -> Option<Frame> {
        self.bot.send_group_forward_msg(self.group_id, message).await
    }

    pub async fn delete_msg(&mut self) -> Option<Frame> {
        self.bot.delete_msg(self.message_id).await
    }

    pub fn get_group_avatar(&self) -> String {
        format!("https://p.qlogo.cn/gh/{0}/{0}/0/", self.group_id)
    }
}
