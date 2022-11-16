use std::f32::consts::E;
use futures_util::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use crate::api::ApiResult;
use crate::core::component::event;
#[derive(Debug)]
pub enum PostType {
    Message(Value),
    Request(Value),
    Notice(Value),
    MetaEvent(Value),
    Null(Value)
}
#[derive(Serialize, Deserialize)]
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
pub enum RequestType {
    Friend,
    Group,
}
#[derive(Serialize, Deserialize)]
pub enum MetaEventType {
    Lifecycle,
    Heartbeat,
}

#[derive(Serialize, Deserialize,Debug)]
pub enum Event{
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
    ApiResult(ApiResult),
}

impl Event{
    pub fn get_event(v:Value) -> Option<Event> {
        let post_type = Self::is(v);
        match post_type {
            PostType::Message(json) => {
                if let Ok(event) = serde_json::from_value::<FriendMessageEvent>(json.clone()){
                  return  Some(Event::FriendMessageEvent(event));
                };
                if let Ok(event) = serde_json::from_value::<GroupMessageEvent>(json.clone()){
                    return  Some(Event::GroupMessageEvent(event));
                };
                None
            }

            PostType::Request(json) => {
                if let Ok(event) = serde_json::from_value::<AddFriendRequest>(json.clone()){
                    return  Some(Event::AddFriendRequest(event));
                };
                if let Ok(event) = serde_json::from_value::<AddGroupRequest>(json.clone()){
                    return  Some(Event::AddGroupRequest(event));
                };
                None
            }

            PostType::Notice(json) => {
                if let Ok(event) = serde_json::from_value::<GroupFileUpload>(json.clone()){
                    return  Some(Event::GroupFileUpload(event));
                };
                if let Ok(event) = serde_json::from_value::<GroupAdminChange>(json.clone()){
                    return  Some(Event::GroupAdminChange(event));
                };
                if let Ok(event) = serde_json::from_value::<GroupMemberDecrease>(json.clone()){
                    return  Some(Event::GroupMemberDecrease(event));
                };
                if let Ok(event) = serde_json::from_value::<GroupMemberIncrease>(json.clone()){
                    return  Some(Event::GroupMemberIncrease(event));
                };
                if let Ok(event) = serde_json::from_value::<GroupBan>(json.clone()){
                    return  Some(Event::GroupBan(event));
                };
                if let Ok(event) = serde_json::from_value::<FriendAdd>(json.clone()){
                    return  Some(Event::FriendAdd(event));
                };
                if let Ok(event) = serde_json::from_value::<FriendMessageRecall>(json.clone()){
                    return  Some(Event::FriendMessageRecall(event));
                };
                if let Ok(event) = serde_json::from_value::<FriendPoke>(json.clone()){
                    return  Some(Event::FriendPoke(event));
                };
                if let Ok(event) = serde_json::from_value::<GroupPoke>(json.clone()){
                    return  Some(Event::GroupPoke(event));
                };
                if let Ok(event) = serde_json::from_value::<TipsForLuckyKingOfRedPackets>(json.clone()){
                    return  Some(Event::TipsForLuckyKingOfRedPackets(event));
                };
                if let Ok(event) = serde_json::from_value::<GroupMemberHonorChangePrompt>(json.clone()){
                    return  Some(Event::GroupMemberHonorChangePrompt(event));
                };
                if let Ok(event) = serde_json::from_value::<GroupMemberBusinessCardUpdate>(json.clone()){
                    return  Some(Event::GroupMemberBusinessCardUpdate(event));
                };
                if let Ok(event) = serde_json::from_value::<OfflineFileReceived>(json.clone()){
                    return  Some(Event::OfflineFileReceived(event));
                };
                if let Ok(event) = serde_json::from_value::<OtherClientOnlineStatusChanges>(json.clone()){
                    return  Some(Event::OtherClientOnlineStatusChanges(event));
                };

                None

            }
            PostType::MetaEvent(json) => {
                // TODO: 心跳什么时都不做
                None
            }
            PostType::Null(json) => {
                if let Ok(event) = serde_json::from_value::<ApiResult>(json.clone()){
                    return  Some(Event::ApiResult(event));
                };
                None
            }
        }

    }
    pub fn is(v: Value) -> PostType {
        let json = v["post_type"].as_str();
        match json{
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

#[derive(Debug,Serialize, Deserialize)]
pub struct FriendSender{
    pub age: i32,
    pub nickname: String,
    pub sex: String,
    pub user_id: i64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupSender{
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
#[derive(Debug,Serialize, Deserialize)]
pub struct Message<M>{
    pub r#type: String,
    pub data: M,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct FriendMessageEvent {
    pub post_type:String,
    pub message_type:String,
    pub time:i64,
    pub self_id:i64,
    pub sub_type:String,
    pub message:Vec<Message<Value>>,
    pub raw_message:String,
    pub font:i32,
    pub sender:FriendSender,
    pub message_id:i64,
    pub user_id:i64,
    pub target_id:i64,
    #[serde(default)]
    pub temp_source:i32,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupMessageEvent{
    pub post_type:String,
    pub message_type:String,
    pub time:i64,
    pub self_id:i64,
    pub sub_type:String,
    pub sender:GroupSender,
    pub user_id:i64,
    pub anonymous:Value,
    pub font:i32,
    pub group_id:i64,
    pub message:Vec<Message<Value>>,
    pub message_seq:i64,
    pub raw_message:String,
    pub message_id:i64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupFileUpload {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub group_id:i64,
    pub user_id:i64,
    pub file:FileInfo,

}

#[derive(Debug,Serialize, Deserialize)]
pub struct FileInfo{
    pub id:String,
    pub name:String,
    pub size:i64,
    pub busid:i64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupAdminChange {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub sub_type:String,
    pub group_id:i64,
    pub user_id:i64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupMemberDecrease {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub sub_type:String,
    pub group_id:i64,
    pub operator_id:i64,
    pub user_id:i64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupMemberIncrease {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub sub_type:String,
    pub group_id:i64,
    pub operator_id:i64,
    pub user_id:i64,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct GroupBan {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub sub_type:String,
    pub group_id:i64,
    pub operator_id:i64,
    pub user_id:i64,
    pub duration:i64,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct FriendAdd {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub user_id:i64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupMessageRecall {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub group_id:i64,
    pub operator_id:i64,
    pub user_id:i64,
    pub message_id:i64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct FriendMessageRecall {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub user_id:i64,
    pub message_id:i64,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct FriendPoke {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub sub_type:String,
    pub sender_id:i64,
    pub user_id:i64,
    pub target_id:i64,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct GroupPoke {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub sub_type:String,
    pub group_id:i64,
    pub user_id:i64,
    pub target_id:i64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct TipsForLuckyKingOfRedPackets {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub group_id:i64,
    pub sub_type:String,
    pub user_id:i64,
    pub target_id:i64,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupMemberHonorChangePrompt {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub group_id:i64,
    pub sub_type:String,
    pub user_id:i64,
    pub honor_type:String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct GroupMemberBusinessCardUpdate {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub group_id:i64,
    pub user_id:i64,
    pub card_new:String,
    pub card_old:String,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct OfflineFileReceived {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub user_id:i64,
    pub file:OfflineFileInfo,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct OfflineFileInfo {
    pub name:String,
    pub size:i64,
    pub url:String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct AddFriendRequest {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub request_type:String,
    pub user_id:i64,
    pub comment:String,
    pub flag:String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct AddGroupRequest {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub request_type:String,
    pub group_id:i64,
    pub user_id:i64,
    pub comment:String,
    pub flag:String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct OtherClientOnlineStatusChanges {
    pub post_type:String,
    pub notice_type:String,
    pub client:Device,
    pub online:bool,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Device {
    pub app_id:i64,
    pub device_name:String,
    pub device_kind:String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct EssenceMessage {
    pub time:i64,
    pub self_id:i64,
    pub post_type:String,
    pub notice_type:String,
    pub sub_type:String,
    pub sender_id:i64,
    pub operator_id:i64,
    pub message_id:i64,
}