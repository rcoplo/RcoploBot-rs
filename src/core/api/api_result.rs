use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::core::event::GroupSender;
use crate::core::message::Message;


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ApiResult {
    pub status: String,
    pub retcode: i32,
    #[serde(default)]
    pub msg: String,
    #[serde(default)]
    pub wording: String,
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub echo: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMsgResult {
    pub message_id: i64,
    pub real_id: i64,
    pub sender: GroupSender,
    pub time: i64,
    pub message: Message,
    pub raw_message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetForwardMsgResult {
    pub messages: Vec<GetForwardMsgMessage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetForwardMsgMessage {
    pub content: String,
    pub sender: GetForwardMsgSender,
    pub time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetForwardMsgSender {
    pub nickname: String,
    pub user_id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetImageResult {
    pub size: i64,
    pub filename: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLoginInfoResult {
    pub user_id: i64,
    pub nickname: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetStrangerInfoResult {
    pub user_id: i64,
    pub nickname: String,
    pub sex: String,
    pub age: i32,
    pub qid: String,
    pub level: i32,
    pub login_days: i32,
}

// 响应为JSON数组,请 使用 Vec<GetFriendListResult> 就可
#[derive(Debug, Deserialize, Serialize)]
pub struct GetFriendListResult {
    pub user_id: i64,
    pub nickname: String,
    pub remark: String,
}
// 响应为JSON数组,请 使用 Vec<GetUnidirectionalFriendListResult>
#[derive(Debug, Deserialize, Serialize)]
pub struct GetUnidirectionalFriendListResult {
    pub user_id: i64,
    pub nickname: String,
    pub remark: String,
}

// 响应为JSON数组,请 使用Vec<GetGroupInfoResult> 就可
#[derive(Debug, Deserialize, Serialize)]
pub struct GetGroupInfoResult {
    pub group_id: i64,
    pub group_name: String,
    pub group_memo: String,
    pub group_create_time: u32,
    pub group_level: u32,
    pub member_count: i32,
    pub max_member_count: i32,
}

// 响应为JSON数组,请 使用 Vec<GetGroupMemberInfoResult> 就可
#[derive(Debug, Deserialize, Serialize)]
pub struct GetGroupMemberInfoResult {
    pub group_id: i64,
    pub user_id: i64,
    pub nickname: String,
    pub card: String,
    pub sex: String,
    pub age: i32,
    #[serde(default)]
    pub area: String,
    pub join_time: i32,
    pub last_sent_time: i32,
    pub level: String,
    pub role: String,
    pub unfriendly: bool,
    #[serde(default)]
    pub title: String,
    pub title_expire_time: i64,
    pub card_changeable: bool,
    pub shut_up_timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetGroupHonorInfoResult {
    pub group_id: i64,
    pub current_talkative: CurrentTalkative,
    pub talkative_list: Vec<HonorInfoList>,
    pub performer_list: Vec<HonorInfoList>,
    pub legend_list: Vec<HonorInfoList>,
    pub strong_newbie_list: Vec<HonorInfoList>,
    pub emotion_list: Vec<HonorInfoList>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentTalkative {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub day_count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HonorInfoList {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetVersionInfoResult {
    pub app_name: String,
    pub app_version: String,
    pub app_full_name: String,
    pub protocol_version: String,
    pub coolq_edition: String,
    pub coolq_directory: String,
    #[serde(rename = "go-cqhttp")]
    pub go_cqhttp: bool,
    pub plugin_version: String,
    pub plugin_build_number: i32,
    pub plugin_build_configuration: String,
    pub runtime_version: String,
    pub runtime_os: String,
    pub version: String,
    pub protocol: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetWordSlicesResult {
    pub slices: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OcrImageResult {
    pub texts: Vec<TextDetection>,
    pub language: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextDetection {
    pub text: String,
    pub confidence: i32,
    pub coordinates: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetGroupSystemMsgResult {
    pub invited_requests: Vec<InvitedRequest>,
    pub join_requests: Vec<JoinRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InvitedRequest {
    pub request_id: i64,
    pub invitor_uin: i64,
    pub invitor_nick: String,
    pub group_id: i64,
    pub group_name: String,
    pub checked: bool,
    pub actor: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRequest {
    pub request_id: i64,
    pub requester_uin: i64,
    pub requester_nick: String,
    pub message: String,
    pub group_id: i64,
    pub group_name: String,
    pub checked: bool,
    pub actor: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetGroupFileSystemInfoRequest {
    pub file_count: i32,
    pub limit_count: i32,
    pub used_space: i64,
    pub total_space: i64,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetGroupRootFilesRequest {
    pub files: Vec<GroupFile>,
    pub folders: Vec<GroupFolder>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetGroupFilesByFolderRequest {
    pub files: Vec<GroupFile>,
    pub folders: Vec<GroupFolder>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroupFile {
    pub group_id: i64,
    pub file_id: String,
    pub file_name: String,
    pub busid: i32,
    pub file_size: i64,
    pub upload_time: i64,
    pub dead_time: i64,
    pub modify_time: i64,
    pub download_times: i64,
    pub uploader: i64,
    pub uploader_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroupFolder {
    pub group_id: i64,
    pub file_id: String,
    pub file_name: String,
    pub create_time: i64,
    pub creator: i64,
    pub creator_name: String,
    pub total_file_count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetStatusRequest {
    pub app_initialized: bool,
    pub app_enabled: bool,
    pub plugins_good: bool,
    pub app_good: bool,
    pub online: bool,
    pub good: bool,
    pub stat: Statistics,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Statistics {
    #[serde(rename = "PacketReceived")]
    pub PacketReceived: u64,
    #[serde(rename = "PacketSent")]
    pub PacketSent: u64,
    #[serde(rename = "PacketLost")]
    pub PacketLost: u32,
    #[serde(rename = "MessageReceived")]
    pub MessageReceived: u64,
    #[serde(rename = "MessageSent")]
    pub MessageSent: u64,
    #[serde(rename = "DisconnectTimes")]
    pub DisconnectTimes: u64,
    #[serde(rename = "LostTimes")]
    pub LostTimes: u64,
    #[serde(rename = "LastMessageTime")]
    pub LastMessageTime: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetGroupAtAllRemainRequest {
    pub can_at_all: bool,
    pub remain_at_all_count_for_group: i16,
    pub remain_at_all_count_for_uin: i16,
}

// 响应为JSON数组,请 使用 Vec<GetGroupNoticeRequest>
#[derive(Debug, Deserialize, Serialize)]
pub struct GetGroupNoticeRequest {
    pub sender_id: i64,
    pub publish_time: i64,
    pub message: NoticeMessage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoticeMessage {
    pub text: String,
    pub images: Vec<NoticeImages>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoticeImages {
    pub height: String,
    pub width: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadFileRequest {
    pub file: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetOnlineClientsRequest {
    pub clients: Vec<Device>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub app_id: i64,
    pub device_name: String,
    pub device_kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGroupMsgHistoryRequest {
    pub message: Vec<Message>,
}
// 响应为JSON数组,请 使用 Vec<GetEssenceMsgListRequest>
#[derive(Debug, Serialize, Deserialize)]
pub struct GetEssenceMsgListRequest {
    pub sender_id: i64,
    pub sender_nick: String,
    pub sender_time: i64,
    pub operator_id: i64,
    pub operator_nick: String,
    pub operator_time: i64,
    pub message_id: i64,
}

//安全等级, 1: 安全 2: 未知 3: 危险
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckUrlSafelyRequest {
    pub level: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetModelShowRequest {
    pub variants: Vec<Variants>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetGroupFileUrlRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variants {
    pub model_show: String,
    pub need_pay: bool,
}






