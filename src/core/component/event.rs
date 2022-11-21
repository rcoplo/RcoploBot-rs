use std::f32::consts::E;
use futures_util::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use crate::api::ApiResult;
use crate::core::component::event;
use crate::core::message::Message;

#[derive(Debug)]
pub enum PostType {
    Message(Value),
    Request(Value),
    Notice(Value),
    MetaEvent(Value),
    Null(Value),
}

#[derive(Serialize, Deserialize)]
pub enum NoticeNotifySubType {
    Honor,
    Poke,
    LuckyKing,
}

#[derive(Serialize, Deserialize)]
pub enum SubType {
    Friend,
    Group,
    GroupSelf,
    Normal,
    Anonymous,
    Notice,
}

#[derive(Serialize, Deserialize)]
pub enum MetaEventType {
    Lifecycle,
    Heartbeat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    FriendMessageEvent(FriendMessageEvent),
    GroupMessageEvent(GroupMessageEvent),
    GroupFileUpload(GroupFileUpload),
    GroupAdminChange(GroupAdminChange),
    GroupMemberDecrease(GroupMemberDecrease),
    GroupMemberIncrease(GroupMemberIncrease),
    GroupBan(GroupBan),
    FriendAdd(FriendAdd),
    GroupMessageRecall(GroupMessageRecall),
    FriendMessageRecall(FriendMessageRecall),
    FriendPoke(FriendPoke),
    GroupPoke(GroupPoke),
    TipsForLuckyKingOfRedPackets(TipsForLuckyKingOfRedPackets),
    GroupMemberHonorChangePrompt(GroupMemberHonorChangePrompt),
    GroupMemberBusinessCardUpdate(GroupMemberBusinessCardUpdate),
    OfflineFileReceived(OfflineFileReceived),
    AddFriendRequest(AddFriendRequest),
    AddGroupRequest(AddGroupRequest),
    OtherClientOnlineStatusChanges(OtherClientOnlineStatusChanges),
    EssenceMessage(EssenceMessage),
    ApiResult(ApiResult),
}

impl Event {
    pub fn get_event(v: Value) -> Option<Event> {
        let post_type = Self::is(v);
        match post_type {
            PostType::Message(json) => {
                let notice_type = json["message_type"].as_str().unwrap_or("null");
                match notice_type {
                    "private" =>{
                        return Some(Event::FriendMessageEvent(serde_json::from_value::<FriendMessageEvent>(json.clone()).unwrap()));
                    }
                    "group" =>{
                        return Some(Event::GroupMessageEvent(serde_json::from_value::<GroupMessageEvent>(json.clone()).unwrap()));
                    }
                    _ => None
                }
            }
            PostType::Request(json) => {
                let notice_type = json["request_type"].as_str().unwrap_or("null");
                match notice_type {
                    "friend" =>{
                        return Some(Event::AddFriendRequest(serde_json::from_value::<AddFriendRequest>(json.clone()).unwrap()));
                    }
                    "group" =>{
                        return Some(Event::AddGroupRequest(serde_json::from_value::<AddGroupRequest>(json.clone()).unwrap()));
                    }
                    _ => None
                }
            }

            PostType::Notice(json) => {
                let notice_type = json["notice_type"].as_str().unwrap_or("null");
                match notice_type {
                    "group_upload" => {
                        return Some(Event::GroupFileUpload(serde_json::from_value::<GroupFileUpload>(json.clone()).unwrap()));
                    }
                    "group_admin" => {
                        return Some(Event::GroupAdminChange(serde_json::from_value::<GroupAdminChange>(json.clone()).unwrap()));
                    }
                    "group_decrease" => {
                        return Some(Event::GroupMemberDecrease(serde_json::from_value::<GroupMemberDecrease>(json.clone()).unwrap()));
                    }
                    "group_increase" => {
                        return Some(Event::GroupMemberIncrease(serde_json::from_value::<GroupMemberIncrease>(json.clone()).unwrap()));
                    }
                    "group_ban" => {
                        return Some(Event::GroupBan(serde_json::from_value::<GroupBan>(json.clone()).unwrap()));
                    }
                    "friend_add" => {
                        return Some(Event::FriendAdd(serde_json::from_value::<FriendAdd>(json.clone()).unwrap()));
                    }
                    "friend_recall" => {
                        return Some(Event::FriendMessageRecall(serde_json::from_value::<FriendMessageRecall>(json.clone()).unwrap()));
                    }
                    "group_recall" => {
                        return Some(Event::GroupMessageRecall(serde_json::from_value::<GroupMessageRecall>(json.clone()).unwrap()));
                    }
                    "notify" => {
                        let sub_type = json["sub_type"].as_str().unwrap_or("null");
                        let group_id = json["group_id"].as_i64().unwrap_or(0);
                        return if sub_type.eq("poke") && group_id == 0 {
                            Some(Event::FriendPoke(serde_json::from_value::<FriendPoke>(json.clone()).unwrap()))
                        } else if sub_type.eq("poke") && group_id != 0 {
                            Some(Event::GroupPoke(serde_json::from_value::<GroupPoke>(json.clone()).unwrap()))
                        } else if sub_type.eq("lucky_king") {
                            Some(Event::TipsForLuckyKingOfRedPackets(serde_json::from_value::<TipsForLuckyKingOfRedPackets>(json.clone()).unwrap()))
                        } else if sub_type.eq("honor") {
                            Some(Event::GroupMemberHonorChangePrompt(serde_json::from_value::<GroupMemberHonorChangePrompt>(json.clone()).unwrap()))
                        } else {
                            None
                        }
                    }
                    "group_card" => {
                        return Some(Event::GroupMemberBusinessCardUpdate(serde_json::from_value::<GroupMemberBusinessCardUpdate>(json.clone()).unwrap()));
                    }
                    "offline_file" => {
                        return Some(Event::OfflineFileReceived(serde_json::from_value::<OfflineFileReceived>(json.clone()).unwrap()));
                    }
                    "client_status" => {
                        return Some(Event::OtherClientOnlineStatusChanges(serde_json::from_value::<OtherClientOnlineStatusChanges>(json.clone()).unwrap()));
                    }
                    "essence" => {
                        return Some(Event::EssenceMessage(serde_json::from_value::<EssenceMessage>(json.clone()).unwrap()));
                    }

                    _ => None
                }

            }
            PostType::MetaEvent(json) => {
                //  心跳什么事都不做
                None
            }
            PostType::Null(json) => {
                if let Ok(event) = serde_json::from_value::<ApiResult>(json.clone()) {
                    return Some(Event::ApiResult(event));
                };
                None
            }
        }
    }
    pub fn is(v: Value) -> PostType {
        let json = v["post_type"].as_str();
        match json {
            None => PostType::Null(v),
            Some(post_type) => {
                match post_type {
                    "message" => PostType::Message(v),
                    "request" => PostType::Request(v),
                    "notice" => PostType::Notice(v),
                    "meta_event" => PostType::MetaEvent(v),
                    _ => PostType::Null(v)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendSender {
    pub age: i32,
    pub nickname: String,
    pub sex: String,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupSender {
    pub age: i32,
    pub area: String,
    pub card: String,
    pub level: String,
    pub nickname: String,
    pub role: String,
    pub sex: String,
    pub title: String,
    pub user_id: i64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendMessageEvent {
    pub post_type: String,
    pub message_type: String,
    pub time: i64,
    pub self_id: i64,
    pub sub_type: String,
    pub message: Vec<Message>,
    pub raw_message: String,
    pub font: i32,
    pub sender: FriendSender,
    pub message_id: i64,
    pub user_id: i64,
    pub target_id: i64,
    #[serde(default)]
    pub temp_source: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMessageEvent {
    pub post_type: String,
    pub message_type: String,
    pub time: i64,
    pub self_id: i64,
    pub sub_type: String,
    pub sender: GroupSender,
    pub user_id: i64,
    pub anonymous: Value,
    pub font: i32,
    pub group_id: i64,
    pub message: Vec<Message>,
    pub message_seq: i64,
    pub raw_message: String,
    pub message_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupFileUpload {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub group_id: i64,
    pub user_id: i64,
    pub file: FileInfo,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub busid: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupAdminChange {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub group_id: i64,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberDecrease {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub group_id: i64,
    pub operator_id: i64,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberIncrease {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub group_id: i64,
    pub operator_id: i64,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupBan {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub group_id: i64,
    pub operator_id: i64,
    pub user_id: i64,
    pub duration: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendAdd {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMessageRecall {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub group_id: i64,
    pub operator_id: i64,
    pub user_id: i64,
    pub message_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendMessageRecall {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub user_id: i64,
    pub message_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendPoke {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub sender_id: i64,
    pub user_id: i64,
    pub target_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupPoke {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub group_id: i64,
    pub user_id: i64,
    pub target_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TipsForLuckyKingOfRedPackets {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub group_id: i64,
    pub sub_type: String,
    pub user_id: i64,
    pub target_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberHonorChangePrompt {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub group_id: i64,
    pub sub_type: String,
    pub user_id: i64,
    pub honor_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberBusinessCardUpdate {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub group_id: i64,
    pub user_id: i64,
    pub card_new: String,
    pub card_old: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfflineFileReceived {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub user_id: i64,
    pub file: OfflineFileInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfflineFileInfo {
    pub name: String,
    pub size: i64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddFriendRequest {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub request_type: String,
    pub user_id: i64,
    pub comment: String,
    pub flag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddGroupRequest {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub request_type: String,
    pub group_id: i64,
    pub user_id: i64,
    pub comment: String,
    pub flag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OtherClientOnlineStatusChanges {
    pub post_type: String,
    pub notice_type: String,
    pub client: Device,
    pub online: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    pub app_id: i64,
    pub device_name: String,
    pub device_kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EssenceMessage {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub sender_id: i64,
    pub operator_id: i64,
    pub message_id: i64,
}