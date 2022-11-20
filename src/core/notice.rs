use crate::api::GetStrangerInfoResult;
use crate::core::bot::{Bot, ResultFrame};
use crate::core::event;
use crate::core::event::{EssenceMessage, FriendAdd, FriendMessageRecall, FriendPoke, GroupAdminChange, GroupBan, GroupFileUpload, GroupMemberBusinessCardUpdate, GroupMemberDecrease, GroupMemberHonorChangePrompt, GroupMemberIncrease, GroupMessageRecall, GroupPoke, OfflineFileReceived, OtherClientOnlineStatusChanges, TipsForLuckyKingOfRedPackets};
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

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
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

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
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

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
    pub async fn get_stranger_info(&mut self) -> Option<GetStrangerInfoResult> {
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

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}


impl Notice<GroupBan> {
    pub fn new_group_ban(event: &GroupBan, bot: &mut Bot) -> Notice<GroupBan> {
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

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}

impl Notice<FriendAdd> {
    pub fn new_friend_add(event: &FriendAdd, bot: &mut Bot) -> Notice<FriendAdd> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: "".to_string(),
            event: event.clone(),
            bot: bot.clone(),
        };
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}

impl Notice<GroupMessageRecall> {
    pub fn new_group_message_recall(event: &GroupMessageRecall, bot: &mut Bot) -> Notice<GroupMessageRecall> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: "".to_string(),
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

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}

impl Notice<FriendMessageRecall> {
    pub fn new_friend_message_recall(event: &FriendMessageRecall, bot: &mut Bot) -> Notice<FriendMessageRecall> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: "".to_string(),
            event: event.clone(),
            bot: bot.clone(),
        };
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}

impl Notice<FriendPoke> {
    pub fn new_friend_poke(event: &FriendPoke, bot: &mut Bot) -> Notice<FriendPoke> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: event.sub_type.clone(),
            event: event.clone(),
            bot: bot.clone(),
        };
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}

impl Notice<GroupPoke> {
    pub fn new_group_poke(event: &GroupPoke, bot: &mut Bot) -> Notice<GroupPoke> {
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

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}

impl Notice<TipsForLuckyKingOfRedPackets> {
    pub fn new_tips_for_lucky_king_of_red_packets(event: &TipsForLuckyKingOfRedPackets, bot: &mut Bot) -> Notice<TipsForLuckyKingOfRedPackets> {
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

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}

impl Notice<GroupMemberHonorChangePrompt> {
    pub fn new_group_member_honor_change_prompt(event: &GroupMemberHonorChangePrompt, bot: &mut Bot) -> Notice<GroupMemberHonorChangePrompt> {
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

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}

impl Notice<GroupMemberBusinessCardUpdate> {
    pub fn new_group_member_business_card_update(event: &GroupMemberBusinessCardUpdate, bot: &mut Bot) -> Notice<GroupMemberBusinessCardUpdate> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: "".to_string(),
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

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}

impl Notice<OfflineFileReceived> {
    pub fn new_offline_file_received(event: &OfflineFileReceived, bot: &mut Bot) -> Notice<OfflineFileReceived> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: "".to_string(),
            event: event.clone(),
            bot: bot.clone(),
        };
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Option<ResultFrame> {
        self.bot.send_private_msg(self.event.user_id, message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Option<ResultFrame> {
        self.bot.send_private_msg_cq(self.event.user_id, message).await
    }
}

impl Notice<OtherClientOnlineStatusChanges> {
    pub fn new_other_client_online_status_changes(event: &OtherClientOnlineStatusChanges, bot: &mut Bot) -> Notice<OtherClientOnlineStatusChanges> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: "".to_string(),
            event: event.clone(),
            bot: bot.clone(),
        };
    }
}
impl Notice<EssenceMessage> {
    pub fn new_essence_message(event: &EssenceMessage, bot: &mut Bot) -> Notice<EssenceMessage> {
        return Notice {
            notice_type: event.notice_type.clone(),
            sub_type: event.sub_type.clone(),
            event: event.clone(),
            bot: bot.clone(),
        };
    }
}

