use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use crate::core::api::api;
use crate::core::api::base_api::BaseApi;
use crate::core::message::Message;


#[derive(Deserialize, Serialize, Clone)]
pub struct SendPrivateMsg {
    pub user_id: i64,
    pub message: Vec<Message>,
    pub auto_escape: bool,
}

impl SendPrivateMsg {
    pub async fn send_private_msg(&self) -> Option<BaseApi<SendPrivateMsg>> {
        let api = BaseApi::new("send_private_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SendPrivateMsgCq {
    pub user_id: i64,
    pub message: String,
    pub auto_escape: bool,
}

impl SendPrivateMsgCq {
    pub async fn send_private_msg_cq(&self) -> Option<BaseApi<SendPrivateMsgCq>> {
        let api = BaseApi::new("send_private_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SendGroupMsg {
    pub group_id: i64,
    pub message: Vec<Message>,
    pub auto_escape: bool,
}

impl SendGroupMsg {
    pub async fn send_group_msg(&self) -> Option<BaseApi<SendGroupMsg>> {
        let api = BaseApi::new("send_group_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SendGroupMsgCq {
    pub group_id: i64,
    pub message: String,
    pub auto_escape: bool,
}

impl SendGroupMsgCq {
    pub async fn send_group_msg_cq(&self) -> Option<BaseApi<SendGroupMsgCq>> {
        let api = BaseApi::new("send_group_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SendGroupForwardMsg {
    pub group_id: i64,
    pub message: Vec<Message>,
}

impl SendGroupForwardMsg {
    pub async fn send_group_forward_msg(&self) -> Option<BaseApi<SendGroupForwardMsg>> {
        let api = BaseApi::new("send_group_forward_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SendMsg {
    pub message_type: String,
    pub group_id: i64,
    pub user_id: i64,
    pub message: Message,
    pub auto_escape: bool,
}

impl SendMsg {
    pub async fn send_msg(&self) -> Option<BaseApi<SendMsg>> {
        let api = BaseApi::new("send_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeleteMsg {
    pub message_id: i64,

}

impl DeleteMsg {
    pub async fn delete_msg(&self) -> Option<BaseApi<DeleteMsg>> {
        let api = BaseApi::new("delete_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetMsg {
    pub message_id: i64,

}

impl GetMsg {
    pub async fn get_msg(&self) -> Option<BaseApi<GetMsg>> {
        let api = BaseApi::new("get_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetForwardMsg {
    pub message_id: i64,

}

impl GetForwardMsg {
    pub async fn get_forward_msg(&self) -> Option<BaseApi<GetForwardMsg>> {
        let api = BaseApi::new("get_forward_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetImage {
    pub file: String,
}

impl GetImage {
    pub async fn get_image(&self) -> Option<BaseApi<GetImage>> {
        let api = BaseApi::new("get_image".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MarkMsgAsRead {
    pub message_id: i64,
}

impl MarkMsgAsRead {
    pub async fn mark_msg_as_read(&self) -> Option<BaseApi<MarkMsgAsRead>> {
        let api = BaseApi::new("mark_msg_as_read".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupKick {
    pub group_id: i64,
    pub user_id: i64,
    pub reject_add_request: bool,
}

impl SetGroupKick {
    pub async fn set_group_kick(&self) -> Option<BaseApi<SetGroupKick>> {
        let api = BaseApi::new("set_group_kick".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupBan {
    pub group_id: i64,
    pub user_id: i64,
    pub duration: Value,
}

impl SetGroupBan {
    pub async fn set_group_ban(&self) -> Option<BaseApi<SetGroupBan>> {
        let api = BaseApi::new("set_group_ban".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupAnonymousBan {
    pub group_id: i64,
    pub flag: String,
    pub duration: Value,
}

impl SetGroupAnonymousBan {
    pub async fn set_group_anonymous_ban(&self) -> Option<BaseApi<SetGroupAnonymousBan>> {
        let api = BaseApi::new("set_group_anonymous_ban".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupWholeBan {
    pub group_id: i64,
    pub enable: bool,
}

impl SetGroupWholeBan {
    pub async fn set_group_whole_ban(&self) -> Option<BaseApi<SetGroupWholeBan>> {
        let api = BaseApi::new("set_group_whole_ban".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupAdmin {
    pub group_id: i64,
    pub user_id: i64,
    pub enable: bool,
}

impl SetGroupAdmin {
    pub async fn set_group_admin(&self) -> Option<BaseApi<SetGroupAdmin>> {
        let api = BaseApi::new("set_group_admin".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupCard {
    pub group_id: i64,
    pub user_id: i64,
    pub card: String,
}

impl SetGroupCard {
    pub async fn set_group_card(&self) -> Option<BaseApi<SetGroupCard>> {
        let api = BaseApi::new("set_group_card".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupName {
    pub group_id: i64,
    pub group_name: String,
}

impl SetGroupName {
    pub async fn set_group_name(&self) -> Option<BaseApi<SetGroupName>> {
        let api = BaseApi::new("set_group_name".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupLeave {
    pub group_id: i64,
    pub is_dismiss: bool,
}

impl SetGroupLeave {
    pub async fn set_group_leave(&self) -> Option<BaseApi<SetGroupLeave>> {
        let api = BaseApi::new("set_group_leave".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupSpecialTitle {
    pub group_id: i64,
    pub user_id: i64,
    pub special_title: bool,
    pub duration: Value,
}

impl SetGroupSpecialTitle {
    pub async fn set_group_special_title(&self) -> Option<BaseApi<SetGroupSpecialTitle>> {
        let api = BaseApi::new("set_group_special_title".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SendGroupSign {
    pub group_id: i64,
}

impl SendGroupSign {
    pub async fn send_group_sign(&self) -> Option<BaseApi<SendGroupSign>> {
        let api = BaseApi::new("send_group_sign".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetFriendAddRequest {
    pub flag: String,
    pub approve: bool,
    pub remark: String,
}

impl SetFriendAddRequest {
    pub async fn set_friend_add_request(&self) -> Option<BaseApi<SetFriendAddRequest>> {
        let api = BaseApi::new("set_friend_add_request".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupAddRequest {
    pub flag: String,
    pub sub_type: String,
    pub approve: bool,
    pub reason: String,
}

impl SetGroupAddRequest {
    pub async fn set_group_add_request(&self) -> Option<BaseApi<SetGroupAddRequest>> {
        let api = BaseApi::new("set_group_add_request".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetLoginInfo;

impl GetLoginInfo {
    pub async fn get_login_info(&self) -> Option<BaseApi<GetLoginInfo>> {
        let api = BaseApi::new("get_login_info".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetQqProfile {
    pub nickname: String,
    pub company: String,
    pub email: String,
    pub college: String,
    pub personal_note: String,

}

impl SetQqProfile {
    pub async fn set_qq_profile(&self) -> Option<BaseApi<SetQqProfile>> {
        let api = BaseApi::new("set_qq_profile".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetStrangerInfo {
    pub user_id: i64,
    pub no_cache: bool,

}

impl GetStrangerInfo {
    pub async fn get_stranger_info(&self) -> Option<BaseApi<GetStrangerInfo>> {
        let api = BaseApi::new("get_stranger_info".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetFriendList;

impl GetFriendList {
    pub async fn get_friend_list(&self) -> Option<BaseApi<GetFriendList>> {
        let api = BaseApi::new("get_friend_list".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetUnidirectionalFriendList;

impl GetUnidirectionalFriendList {
    pub async fn get_unidirectional_friend_list(&self) -> Option<BaseApi<GetUnidirectionalFriendList>> {
        let api = BaseApi::new("get_unidirectional_friend_list".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeleteFriend {
    pub friend_id: i64,
}

impl DeleteFriend {
    pub async fn delete_friend(&self) -> Option<BaseApi<DeleteFriend>> {
        let api = BaseApi::new("delete_friend".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupInfo {
    pub group_id: i64,
    pub no_cache: bool,
}

impl GetGroupInfo {
    pub async fn get_group_info(&self) -> Option<BaseApi<GetGroupInfo>> {
        let api = BaseApi::new("get_group_info".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupList;

impl GetGroupList {
    pub async fn get_group_list(&self) -> Option<BaseApi<GetGroupList>> {
        let api = BaseApi::new("get_group_list".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupMemberInfo {
    pub group_id: i64,
    pub user_id: i64,
    pub no_cache: bool,
}

impl GetGroupMemberInfo {
    pub async fn get_group_member_info(&self) -> Option<BaseApi<GetGroupMemberInfo>> {
        let api = BaseApi::new("get_group_member_info".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupMemberList {
    pub group_id: i64,
    pub no_cache: bool,
}

impl GetGroupMemberList {
    pub async fn get_group_member_list(&self) -> Option<BaseApi<GetGroupMemberList>> {
        let api = BaseApi::new("get_group_member_list".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupHonorInfo {
    pub group_id: i64,
    #[serde(rename(serialize = "type"))]
    pub r#type: String,
}

impl GetGroupHonorInfo {
    pub async fn get_group_honor_info(&self) -> Option<BaseApi<GetGroupHonorInfo>> {
        let api = BaseApi::new("get_group_honor_info".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CanSendImage;

impl CanSendImage {
    pub async fn can_send_image(&self) -> Option<BaseApi<CanSendImage>> {
        let api = BaseApi::new("can_send_image".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CanSendRecord;

impl CanSendRecord {
    pub async fn can_send_record(&self) -> Option<BaseApi<CanSendRecord>> {
        let api = BaseApi::new("can_send_record".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetVersionInfo;

impl GetVersionInfo {
    pub async fn get_version_info(&self) -> Option<BaseApi<GetVersionInfo>> {
        let api = BaseApi::new("get_version_info".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetRestart {
    pub delay: Value,
}

impl SetRestart {
    pub async fn set_restart(&self) -> Option<BaseApi<SetRestart>> {
        let api = BaseApi::new("set_restart".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetGroupPortrait {
    pub group_id: i64,
    pub file: String,
    pub cache: i32,
}

impl SetGroupPortrait {
    pub async fn set_group_portrait(&self) -> Option<BaseApi<SetGroupPortrait>> {
        let api = BaseApi::new("set_group_portrait".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetWordSlices {
    pub content: String,
}

impl GetWordSlices {
    pub async fn get_word_slices(&self) -> Option<BaseApi<GetWordSlices>> {
        let api = BaseApi::new(".get_word_slices".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct OcrImage {
    pub image: String,
}

impl OcrImage {
    pub async fn ocr_image(&self) -> Option<BaseApi<OcrImage>> {
        let api = BaseApi::new("ocr_image".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupSystemMsg;

impl GetGroupSystemMsg {
    pub async fn get_group_system_msg(&self) -> Option<BaseApi<GetGroupSystemMsg>> {
        let api = BaseApi::new("get_group_system_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UploadPrivateFile {
    pub user_id: i64,
    pub file: String,
    pub name: String,
}

impl UploadPrivateFile {
    pub async fn upload_private_file(&self) -> Option<BaseApi<UploadPrivateFile>> {
        let api = BaseApi::new("upload_private_file".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UploadGroupFile {
    pub group_id: i64,
    pub file: String,
    pub name: String,
    pub folder: String,
}

impl UploadGroupFile {
    pub async fn upload_group_file(&self) -> Option<BaseApi<UploadGroupFile>> {
        let api = BaseApi::new("upload_group_file".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupFileSystemInfo {
    pub group_id: i64,
}

impl GetGroupFileSystemInfo {
    pub async fn get_group_file_system_info(&self) -> Option<BaseApi<GetGroupFileSystemInfo>> {
        let api = BaseApi::new("get_group_file_system_info".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupRootFiles {
    pub group_id: i64,
}

impl GetGroupRootFiles {
    pub async fn get_group_root_files(&self) -> Option<BaseApi<GetGroupRootFiles>> {
        let api = BaseApi::new("get_group_root_files".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupFilesByFolder {
    pub group_id: i64,
    pub folder_id: String,
}

impl GetGroupFilesByFolder {
    pub async fn get_group_files_by_folder(&self) -> Option<BaseApi<GetGroupFilesByFolder>> {
        let api = BaseApi::new("get_group_files_by_folder".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateGroupFileFolder {
    pub group_id: i64,
    pub name: String,
    pub parent_id: String,
}

impl CreateGroupFileFolder {
    pub async fn create_group_file_folder(&self) -> Option<BaseApi<CreateGroupFileFolder>> {
        let api = BaseApi::new("create_group_file_folder".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeleteGroupFolder {
    pub group_id: i64,
    pub folder_id: String,
}

impl DeleteGroupFolder {
    pub async fn delete_group_folder(&self) -> Option<BaseApi<DeleteGroupFolder>> {
        let api = BaseApi::new("delete_group_folder".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeleteGroupFile {
    pub group_id: i64,
    pub file_id: String,
    pub busid: i32,
}

impl DeleteGroupFile {
    pub async fn delete_group_file(&self) -> Option<BaseApi<DeleteGroupFile>> {
        let api = BaseApi::new("delete_group_file".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupFileUrl {
    pub group_id: i64,
    pub file_id: String,
    pub busid: i32,
}

impl GetGroupFileUrl {
    pub async fn get_group_file_url(&self) -> Option<BaseApi<GetGroupFileUrl>> {
        let api = BaseApi::new("get_group_file_url".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetStatus;

impl GetStatus {
    pub async fn get_status(&self) -> Option<BaseApi<GetStatus>> {
        let api = BaseApi::new("get_status".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupAtAllRemain {
    pub group_id: i64,
}

impl GetGroupAtAllRemain {
    pub async fn get_group_at_all_remain(&self) -> Option<BaseApi<GetGroupAtAllRemain>> {
        let api = BaseApi::new("get_group_at_all_remain".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct HandleQuickOperation {
    pub context: Value,
    pub operation: Value,
}

impl HandleQuickOperation {
    pub async fn handle_quick_operation(&self) -> Option<BaseApi<HandleQuickOperation>> {
        let api = BaseApi::new(".handle_quick_operation".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SendGroupNotice {
    pub group_id: i64,
    pub content: String,
    pub image: String,
}

impl SendGroupNotice {
    pub async fn _send_group_notice(&self) -> Option<BaseApi<SendGroupNotice>> {
        let api = BaseApi::new("_send_group_notice".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupNotice {
    pub group_id: i64,
}

impl GetGroupNotice {
    pub async fn _get_group_notice(&self) -> Option<BaseApi<GetGroupNotice>> {
        let api = BaseApi::new("_send_group_notice".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ReloadEventFilter {
    pub file: String,
}

impl ReloadEventFilter {
    pub async fn reload_event_filter(&self) -> Option<BaseApi<ReloadEventFilter>> {
        let api = BaseApi::new("reload_event_filter".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DownloadFile {
    pub url: String,
    pub thread_count: i32,
    pub headers: Vec<String>,
}

impl DownloadFile {
    pub async fn download_file(&self) -> Option<BaseApi<DownloadFile>> {
        let api = BaseApi::new("download_file".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetOnlineClients {
    pub no_cache: bool,
}

impl GetOnlineClients {
    pub async fn get_online_clients(&self) -> Option<BaseApi<GetOnlineClients>> {
        let api = BaseApi::new("get_online_clients".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetGroupMsgHistory {
    pub message_seq: i64,
    pub group_id: i64,
}

impl GetGroupMsgHistory {
    pub async fn get_group_msg_history(&self) -> Option<BaseApi<GetGroupMsgHistory>> {
        let api = BaseApi::new("get_group_msg_history".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetEssenceMsg {
    pub message_id: i64,
}

impl SetEssenceMsg {
    pub async fn set_essence_msg(&self) -> Option<BaseApi<SetEssenceMsg>> {
        let api = BaseApi::new("set_essence_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeleteEssenceMsg {
    pub message_id: i64,
}

impl DeleteEssenceMsg {
    pub async fn delete_essence_msg(&self) -> Option<BaseApi<DeleteEssenceMsg>> {
        let api = BaseApi::new("delete_essence_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetEssenceMsgList {
    pub group_id: i64,
}

impl GetEssenceMsgList {
    pub async fn get_essence_msg_list(&self) -> Option<BaseApi<GetEssenceMsgList>> {
        let api = BaseApi::new("get_essence_msg_list".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CheckUrlSafely {
    pub url: String,
}

impl CheckUrlSafely {
    pub async fn check_url_safely(&self) -> Option<BaseApi<CheckUrlSafely>> {
        let api = BaseApi::new("check_url_safely".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetModelShow {
    pub model: String,
}

impl GetModelShow {
    pub async fn _get_model_show(&self) -> Option<BaseApi<GetModelShow>> {
        let api = BaseApi::new("_get_model_show".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SetModelShow {
    pub model: String,
    pub model_show: String,
}

impl SetModelShow {
    pub async fn _set_model_show(&self) -> Option<BaseApi<SetModelShow>> {
        let api = BaseApi::new("_set_model_show".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeleteUnidirectionalFriend {
    pub user_id: i64,
}

impl DeleteUnidirectionalFriend {
    pub async fn delete_unidirectional_friend(&self) -> Option<BaseApi<DeleteUnidirectionalFriend>> {
        let api = BaseApi::new("delete_unidirectional_friend".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SendPrivateForwardMsg {
    pub user_id: i64,
    pub message: Message,
}

impl SendPrivateForwardMsg {
    pub async fn send_private_forward_msg(&self) -> Option<BaseApi<SendPrivateForwardMsg>> {
        let api = BaseApi::new("send_private_forward_msg".to_string(), Self {
            ..self.clone()
        });
        match api {
            None => None,
            Some(api) => Some(api)
        }
    }
}

