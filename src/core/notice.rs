use futures_util::SinkExt;
use crate::api::GetStrangerInfoResult;
use crate::core::bot::{Bot, ResultFrame};
use crate::core::event;
use crate::core::event::{GroupAdminChange, GroupBan, GroupFileUpload, GroupMemberDecrease, GroupMemberIncrease};
use crate::core::message::Message;

#[derive(Debug, Clone)]
pub struct Notice<E> {
    pub notice_type: String,
    pub sub_type: String,
    pub event: E,
    pub bot: Bot,
}

impl Notice<GroupFileUpload> {
    pub fn new_group_file_upload(event: &GroupFileUpload, bot: &mut Bot) -> Notice<GroupFileUpload> {
        Notice {
            notice_type: event.notice_type.clone(),
            sub_type: "".to_string(),
            event: event.clone(),
            bot: bot.clone(),
        }
    }
    pub async fn send_group_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_group_msg(self.event.group_id, message).await
    }

    pub async fn send_group_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_group_msg_cq(self.event.group_id, message).await
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>, ) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id,message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id,message).await
    }
}

impl Notice<GroupAdminChange> {
    pub fn new_group_admin_change(event: &GroupAdminChange, bot: &mut Bot) -> Notice<GroupAdminChange> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: event.sub_type.clone(),
            event: event.clone(),
            bot: bot.clone(),
        };
    }
    pub async fn send_group_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_group_msg(self.event.group_id, message).await
    }

    pub async fn send_group_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_group_msg_cq(self.event.group_id, message).await
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>, ) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id,message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id,message).await
    }
}
impl Notice<GroupMemberDecrease> {
    pub fn new_group_member_decrease(event: &GroupMemberDecrease, bot: &mut Bot) -> Notice<GroupMemberDecrease> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: event.sub_type.clone(),
            event: event.clone(),
            bot: bot.clone(),
        };
    }
    pub async fn send_group_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_group_msg(self.event.group_id, message).await
    }

    pub async fn send_group_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_group_msg_cq(self.event.group_id, message).await
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>, ) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id,message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id,message).await
    }
    pub async fn get_stranger_info(&mut self ) -> Option<GetStrangerInfoResult> {
        self.bot.get_stranger_info(self.event.user_id).await
    }
}
impl Notice<GroupMemberIncrease> {
    pub fn new_group_member_increase(event: &GroupMemberIncrease, bot: &mut Bot) -> Notice<GroupMemberIncrease> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: event.sub_type.clone(),
            event: event.clone(),
            bot: bot.clone(),
        };
    }
    pub async fn send_group_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_group_msg(self.event.group_id, message).await
    }

    pub async fn send_group_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_group_msg_cq(self.event.group_id, message).await
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>, ) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id,message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id,message).await
    }
}
// impl GroupFileUpload {
//    pub fn new(event: &GroupFileUpload, bot: &mut Bot) -> Notice<GroupFileUpload> {
//         Notice {
//             notice_type: event.notice_type.clone(),
//             sub_type: "".to_string(),
//             event: event.clone(),
//             bot: bot.clone(),
//         }
//     }
//
// }

// impl GroupAdminChange {
//     pub fn new(event: &GroupAdminChange, bot: &mut Bot) -> Notice<GroupAdminChange> {
//         return Notice {
//             notice_type: event.notice_type.clone(),
//             sub_type: event.sub_type.clone(),
//             event: event.clone(),
//             bot: bot.clone(),
//         };
//     }
// }
// impl GroupMemberDecrease {
//     pub fn new(event: &GroupMemberDecrease, bot: &mut Bot) -> Notice<GroupMemberDecrease> {
//         return Notice {
//             notice_type: event.notice_type.clone(),
//             sub_type: event.sub_type.clone(),
//             event: event.clone(),
//             bot: bot.clone(),
//         };
//     }
// }
//
// impl GroupMemberIncrease {
//     pub fn new(event: &GroupMemberIncrease, bot: &mut Bot) -> Notice<GroupMemberIncrease> {
//         return Notice {
//             notice_type: event.notice_type.clone(),
//             sub_type: event.sub_type.clone(),
//             event: event.clone(),
//             bot: bot.clone(),
//         };
//     }
// }

impl GroupBan {
    pub fn new(event: &GroupBan, bot: &mut Bot) -> Notice<GroupBan> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: event.sub_type.clone(),
            event: event.clone(),
            bot: bot.clone(),
        };
    }
}



// Event::FriendAdd(event) => {
//     Self {
//         notice_type: NoticeType::FriendAdd,
//         event,
//     }
// }
// Event::GroupMessageRecall(event) => {
//     Self {
//         notice_type: NoticeType::GroupRecall,
//         event,
//     }
// }
// Event::FriendMessageRecall(event) => {
//     Self {
//         notice_type: NoticeType::FriendRecall,
//         event,
//     }
// }
// Event::FriendPoke(event) => {
//     Self {
//         notice_type: NoticeType::Notify,
//         event,
//     }
// }
// Event::GroupPoke(event) => {
//     Self {
//         notice_type: NoticeType::Notify,
//         event,
//     }
// }
// Event::TipsForLuckyKingOfRedPackets(event) => {
//     Self {
//         notice_type: NoticeType::GroupUpload,
//         event,
//     }
// }
// Event::GroupMemberHonorChangePrompt(event) => {
//     Self {
//         notice_type: NoticeType::GroupUpload,
//         event,
//     }
// }
// Event::GroupMemberBusinessCardUpdate(event) => {
//     Self {
//         notice_type: NoticeType::GroupUpload,
//         event,
//     }
// }
// Event::OfflineFileReceived(event) => {
//     Self {
//         notice_type: NoticeType::GroupUpload,
//         event,
//     }
// }
// Event::OtherClientOnlineStatusChanges(event) => {
//     Self {
//         notice_type: NoticeType::GroupUpload,
//         event,
//     }
// }
//             _ => {
//                 Self{
//                     notice_type: NoticeType::Null,
//                     sub_type: NoticeSubType::Null,
//                     event: Box::new(event.clone())
//                 }
//
//             }
//         }
//
//     }
// }
#[derive(Debug, Clone)]
pub enum NoticeType {
    GroupUpload,
    GroupAdmin,
    GroupDecrease,
    GroupIncrease,
    GroupBan,
    FriendAdd,
    GroupRecall,
    FriendRecall,
    GroupCard,
    OfflineFile,
    ClientStatus,
    Essence,
    Notify,
    Null,
}

#[derive(Debug, Clone)]
pub enum NoticeSubType {
    Null,
    Set,
    Unset,
    Leave,
    Kick,
    KickMe,
    Approve,
    Invite,
    Ban,
    LiftBan,
    Poke,
    LuckyKing,
    Honor,
    Add,
    Delete,
}
