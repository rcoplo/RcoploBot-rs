use log::info;
use crate::core::api::GetStrangerInfoResult;
use crate::core::bot::{Bot, message_handle, ResultFrame};
use crate::core::event::{Event, FriendMessageEvent, FriendSender};
use crate::core::message::{Message, message_type_handle};

#[derive(Debug,Clone)]
pub struct Friend{
    //QQ号
    pub user_id:i64,
    //消息id
    pub message_id:i64,
    //格式化的消息
    pub message:String,
    //以空格分组的消息
    pub message_list:Vec<String>,
    //好友的信息
    pub sender:FriendSender,
    //bot
    bot:Bot,
}

impl Friend {
    pub fn new(event: &FriendMessageEvent, bot:&mut Bot) -> Self{
        let (message,message_list) = message_handle(event.message.clone(), event.raw_message.clone());
        info!("Q::{} > {:?}",&event.user_id,&message);
        Self {
            user_id: event.user_id.clone(),
            message_id: event.message_id.clone(),
            message,
            message_list,
            sender: event.sender.clone(),
            bot: bot.clone(),
        }
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>, ) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.user_id,message).await
    }

    pub async fn send_private_msg_cq(&mut self, user_id: i64, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.user_id,message).await
    }

    pub async fn delete_msg(&mut self) -> Option<ResultFrame> {
        self.bot.delete_msg(self.message_id).await
    }
    pub async fn get_stranger_info(&mut self ) -> Option<GetStrangerInfoResult> {
        self.bot.get_stranger_info(self.user_id).await
    }
    pub fn get_user_avatar(&self) -> String {
        format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=0", self.user_id)
    }

}