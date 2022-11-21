use log::info;
use crate::api::GetStrangerInfoResult;
use crate::core::bot::{Bot, ResultFrame};
use crate::core::event::{Event, GroupMessageEvent, GroupSender};
use crate::core::message::{Message, message_type_handle};


#[derive(Debug,Clone)]
pub struct Group {
    pub group_id: i64,
    pub user_id: i64,
    pub message_id: i64,
    pub message:String,
    pub message_list:Vec<String>,
    pub sender: GroupSender,
    pub bot: Bot,
}

impl Group {
    pub fn new(event: &GroupMessageEvent, bot:&mut Bot) -> Self {
        let mut vec = vec![];
        let message = message_type_handle(event.message.clone());
        let binding = event.raw_message.clone();
        let msg_list:Vec<_> = binding.split_whitespace().collect();
        for msg in msg_list {
            vec.push(msg.to_string());
        }
        info!("G::{} >Q::{} > {:?}",&event.group_id,&event.user_id,&message);
        Self {
            group_id: event.group_id.clone(),
            user_id:event.user_id.clone(),
            message_id: event.message_id.clone(),
            message,
            message_list: vec,
            sender: event.sender.clone(),
            bot: bot.clone(),
        }
    }
    pub async fn send_group_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_group_msg(self.group_id, message).await
    }

    pub async fn send_group_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_group_msg_cq(self.group_id, message).await
    }

    pub async fn send_group_forward_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_group_forward_msg(self.group_id, message).await
    }

    pub async fn delete_msg(&mut self,message_id:i64) -> Option<ResultFrame> {
        self.bot.delete_msg(message_id).await
    }

    pub async fn get_stranger_info(&mut self ) -> Option<GetStrangerInfoResult> {
        self.bot.get_stranger_info(self.user_id).await
    }

    pub fn get_group_avatar(&self) -> String {
        format!("https://p.qlogo.cn/gh/{0}/{0}/0/", self.group_id)
    }
}
