use log::info;
use crate::core::bot::{Bot, ResultFrame};
use crate::core::event::{AddFriendRequest, AddGroupRequest, Event};
use crate::core::message::Message;
#[derive(Debug)]
pub struct Request {
    pub request_type: RequestType,
    pub group_id: i64,
    pub user_id: i64,
    pub comment: String,
    pub flag: String,
    bot: Bot,
}

impl Request {
    pub fn new_add_friend(event: &AddFriendRequest, bot: &mut Bot) -> Self {
        let request = Self {
            request_type: RequestType::Friend,
            group_id: 0,
            user_id: event.user_id.clone(),
            comment: event.comment.clone(),
            flag: event.flag.clone(),
            bot: bot.clone(),
        };
        info!("Q::{} > {:?}",&event.user_id,&request);
        request
    }
    pub fn new_add_group(event: &AddGroupRequest, bot: &mut Bot) -> Self {
        let request = Self {
            request_type: RequestType::Group,
            group_id: event.group_id.clone(),
            user_id: event.user_id.clone(),
            comment: event.comment.clone(),
            flag: event.flag.clone(),
            bot: bot.clone(),
        };
        info!("G::{} > Q::{} > {:?}",&event.group_id,&event.user_id,&request);
        request
    }

    pub async fn set_friend_add_request(&mut self, approve: bool, remark: &str) -> Option<ResultFrame> {
        self.bot.set_friend_add_request(self.flag.as_str(), approve, remark).await
    }
    pub async fn set_group_add_request(&mut self, sub_type: GroupAddSubType, approve: bool, reason: &str) -> Option<ResultFrame> {
        match sub_type {
            GroupAddSubType::Add => {
                self.bot.set_group_add_request(self.flag.as_str(), "add", approve, reason).await
            }
            GroupAddSubType::Invite => {
                self.bot.set_group_add_request(self.flag.as_str(), "invite", approve, reason).await
            }
        }
    }
    pub async fn send_group_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_group_msg(self.group_id, message).await
    }

    pub async fn send_group_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_group_msg_cq(self.group_id, message).await
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>, ) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.user_id,message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.user_id,message).await
    }

}
#[derive(Debug)]
pub enum GroupAddSubType {
    Add,
    Invite,
}
#[derive(Debug)]
pub enum RequestType {
    Friend,
    Group,
    Null,
}