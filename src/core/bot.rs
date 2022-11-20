use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use futures_util::lock::Mutex;
use futures_util::{SinkExt, StreamExt};
use log::info;

use serde_json::{Error, json, Map, Value};
use serde::{Deserialize, Serialize};
use crate::api::*;

use crate::core::component::message::Message;


#[derive(Debug, Clone)]
pub struct Bot {
    pub bot_id: i64,
    pub bot_name: String,
    pub api_sender: mpsc::Sender<ResultFrame>,
    pub resp_promises: Arc<Mutex<HashMap<String, oneshot::Sender<ResultFrame>>>>,
}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct ResultFrame {
    pub bot_id: i64,
    pub echo: String,
    pub ok: bool,
    pub data: Option<Value>,
    pub message_id: i64,
}


fn base_api_to_json<P: Serialize>(base_api: BaseApi<P>) -> Value {
    serde_json::to_value(&base_api).unwrap()
}

impl Bot {
    pub async fn send_and_wait<P: Serialize>(&mut self, api: BaseApi<P>) -> Option<ResultFrame> {
        let echo: String = uuid::Uuid::new_v4().to_string();
        let api = api;
        let data = BaseApi {
            echo: echo.clone(),
            ..api
        };
        let data = base_api_to_json(data);
        let frame = ResultFrame {
            bot_id: self.bot_id,
            echo: echo.clone(),
            ok: true,
            data: Some(data),
            message_id: 0,
        };
        // 发送API请求
        let api_sender = mpsc::Sender::clone(&self.api_sender);
        api_sender.send(frame).await;

        // 等待API响应
        let (resp_sender, mut resp_receiver) = oneshot::channel();
        self.resp_promises.lock().await.insert(echo.clone(), resp_sender);
        let api_resp_frame = resp_receiver.await.unwrap();
        Some(api_resp_frame)
    }


    pub async fn send_private_msg(
        &mut self,
        user_id: i64,
        message: Vec<Message>,
    ) -> Option<ResultFrame> {
        let re = SendPrivateMsg {
            user_id,
            message,
            auto_escape: false,
        };
        let api = re.send_private_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }

    pub async fn send_private_msg_cq(&mut self, user_id: i64, message: String) -> Option<ResultFrame> {
        let re = SendPrivateMsgCq {
            user_id,
            message: message.to_string(),
            auto_escape: false,
        };
        let api = re.send_private_msg_cq().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }

    pub async fn send_group_msg(&mut self, group_id: i64, message: Vec<Message>, ) -> Option<ResultFrame> {
        let re = SendGroupMsg {
            group_id,
            message,
            auto_escape: false,
        };
        let api = re.send_group_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn send_group_msg_cq(&mut self, group_id: i64, message: String) -> Option<ResultFrame> {
        let re = SendGroupMsgCq {
            group_id,
            message: message.to_string(),
            auto_escape: false,
        };
        let api = re.send_group_msg_cq().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn send_group_forward_msg(&mut self, group_id: i64, message: Vec<Message>) -> Option<ResultFrame> {
        let re = SendGroupForwardMsg {
            group_id,
            message,
        };
        let api = re.send_group_forward_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn send_msg(&mut self, message_type: &str, group_id: i64, user_id: i64, message: Message) -> Option<ResultFrame> {
        let re = SendMsg {
            message_type: message_type.to_string(),
            group_id,
            user_id,
            message,
            auto_escape: false,
        };
        let api = re.send_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }

    pub async fn delete_msg(&mut self, message_id: i64) -> Option<ResultFrame> {
        let re = DeleteMsg {
            message_id,
        };
        let api = re.delete_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_msg(&mut self, message_id: i64) -> Option<GetMsgResult> {
        let re = GetMsg {
            message_id,
        };
        let api = re.get_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetMsgResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_forward_msg(&mut self, message_id: i64) -> Option<GetForwardMsgResult> {
        let re = GetForwardMsg {
            message_id,
        };
        let api = re.get_forward_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetForwardMsgResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_image(&mut self, file: &str) -> Option<GetImageResult> {
        let re = GetImage {
            file: file.to_string(),
        };
        let api = re.get_image().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetImageResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }

    pub async fn can_send_image(&mut self) -> Option<ResultFrame> {
        let api = CanSendImage.can_send_image().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn mark_msg_as_read(&mut self, message_id: i64) -> Option<ResultFrame> {
        let re = MarkMsgAsRead {
            message_id,
        };
        let api = re.mark_msg_as_read().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_kick(
        &mut self,
        group_id: i64,
        user_id: i64,
        reject_add_request: bool,
    ) -> Option<ResultFrame> {
        let re = SetGroupKick {
            group_id,
            user_id,
            reject_add_request,
        };
        let api = re.set_group_kick().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_ban(
        &mut self,
        group_id: i64,
        user_id: i64,
        duration: Value,
    ) -> Option<ResultFrame> {
        let re = SetGroupBan {
            group_id,
            user_id,
            duration,
        };
        let api = re.set_group_ban().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_anonymous_ban(
        &mut self,
        group_id: i64,
        flag: &str,
        duration: Value,
    ) -> Option<ResultFrame> {
        let re = SetGroupAnonymousBan {
            group_id,
            flag: flag.to_string(),
            duration,
        };
        let api = re.set_group_anonymous_ban().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_whole_ban(
        &mut self,
        group_id: i64,
        enable: bool,
    ) -> Option<ResultFrame> {
        let re = SetGroupWholeBan {
            group_id,
            enable,
        };
        let api = re.set_group_whole_ban().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_admin(
        &mut self,
        group_id: i64,
        user_id: i64,
        enable: bool,
    ) -> Option<ResultFrame> {
        let re = SetGroupAdmin {
            group_id,
            user_id,
            enable,
        };
        let api = re.set_group_admin().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_card(
        &mut self,
        group_id: i64,
        user_id: i64,
        card: &str,
    ) -> Option<ResultFrame> {
        let re = SetGroupCard {
            group_id,
            user_id,
            card: card.to_string(),
        };
        let api = re.set_group_card().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_name(
        &mut self,
        group_id: i64,
        group_name: &str,
    ) -> Option<ResultFrame> {
        let re = SetGroupName {
            group_id,
            group_name: group_name.to_string(),
        };
        let api = re.set_group_name().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_leave(
        &mut self,
        group_id: i64,
        is_dismiss: bool,
    ) -> Option<ResultFrame> {
        let re = SetGroupLeave {
            group_id,
            is_dismiss,
        };
        let api = re.set_group_leave().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_special_title(
        &mut self,
        group_id: i64,
        user_id: i64,
        special_title: bool,
        duration: Value,
    ) -> Option<ResultFrame> {
        let re = SetGroupSpecialTitle {
            group_id,
            user_id,
            special_title,
            duration,
        };
        let api = re.set_group_special_title().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn send_group_sign(
        &mut self,
        group_id: i64,
    ) -> Option<ResultFrame> {
        let re = SendGroupSign {
            group_id,
        };
        let api = re.send_group_sign().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_friend_add_request(
        &mut self,
        flag: &str,
        approve: bool,
        remark: &str,
    ) -> Option<ResultFrame> {
        let re = SetFriendAddRequest {
            flag: flag.to_string(),
            approve,
            remark: remark.to_string(),
        };
        let api = re.set_friend_add_request().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_add_request(
        &mut self,
        flag: &str,
        sub_type: &str,
        approve: bool,
        reason: &str,
    ) -> Option<ResultFrame> {
        let re = SetGroupAddRequest {
            flag: flag.to_string(),
            sub_type: sub_type.to_string(),
            approve,
            reason: reason.to_string(),
        };
        let api = re.set_group_add_request().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_login_info(
        &mut self,
    ) -> Option<GetLoginInfoResult> {
        let api = GetLoginInfo.get_login_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetLoginInfoResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn set_qq_profile(
        &mut self,
        nickname: &str,
        company: &str,
        email: &str,
        college: &str,
        personal_note: &str,
    ) -> Option<ResultFrame> {
        let re = SetQqProfile {
            nickname: nickname.to_string(),
            company: company.to_string(),
            email: email.to_string(),
            college: college.to_string(),
            personal_note: personal_note.to_string(),
        };
        let api = re.set_qq_profile().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_stranger_info(
        &mut self,
        user_id: i64,
    ) -> Option<GetStrangerInfoResult> {
        let re = GetStrangerInfo {
            user_id,
            no_cache: false,
        };
        let api = re.get_stranger_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetStrangerInfoResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_friend_list(
        &mut self,
    ) -> Option<Vec<GetFriendListResult>> {
        let api = GetFriendList.get_friend_list().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<Vec<GetFriendListResult>>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_unidirectional_friend_list(
        &mut self,
    ) -> Option<Vec<GetUnidirectionalFriendListResult>> {
        let api = GetUnidirectionalFriendList.get_unidirectional_friend_list().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<Vec<GetUnidirectionalFriendListResult>>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn delete_friend(
        &mut self,
        friend_id: i64,
    ) -> Option<ResultFrame> {
        let re = DeleteFriend {
            friend_id,
        };
        let api = re.delete_friend().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_group_info(
        &mut self,
        group_id: i64,
    ) -> Option<GetGroupInfoResult> {
        let re = GetGroupInfo {
            group_id,
            no_cache: false,
        };
        let api = re.get_group_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupInfoResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_group_list(
        &mut self,
    ) -> Option<Vec<GetGroupInfoResult>> {
        let api = GetGroupList.get_group_list().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<Vec<GetGroupInfoResult>>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_group_member_info(
        &mut self,
        group_id: i64,
        user_id: i64,
    ) -> Option<GetGroupMemberInfoResult> {
        let re = GetGroupMemberInfo {
            group_id,
            user_id,
            no_cache: false,
        };
        let api = re.get_group_member_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupMemberInfoResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_group_member_list(
        &mut self,
        group_id: i64,
    ) -> Option<Vec<GetGroupMemberInfoResult>> {
        let re = GetGroupMemberList {
            group_id,
            no_cache: false,
        };
        let api = re.get_group_member_list().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<Vec<GetGroupMemberInfoResult>>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_group_honor_info(
        &mut self,
        group_id: i64,
        r#type: &str,
    ) -> Option<GetGroupHonorInfoResult> {
        let re = GetGroupHonorInfo {
            group_id,
            r#type: r#type.to_string(),
        };
        let api = re.get_group_honor_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupHonorInfoResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn can_send_record(
        &mut self,
    ) -> Option<ResultFrame> {
        let api = CanSendRecord.can_send_record().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_version_info(
        &mut self,
    ) -> Option<GetVersionInfoResult> {
        let api = GetVersionInfo.get_version_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetVersionInfoResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn set_restart(
        &mut self,
        delay: Value,
    ) -> Option<ResultFrame> {
        let re = SetRestart {
            delay,
        };
        let api = re.set_restart().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn set_group_portrait(
        &mut self,
        group_id: i64,
        file: &str,
        cache: i32,
    ) -> Option<ResultFrame> {
        let re = SetGroupPortrait {
            group_id,
            file: file.to_string(),
            cache,
        };
        let api = re.set_group_portrait().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_word_slices(
        &mut self,
        content: &str,
    ) -> Option<GetWordSlicesResult> {
        let re = GetWordSlices {
            content: content.to_string(),
        };
        let api = re.get_word_slices().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetWordSlicesResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn ocr_image(
        &mut self,
        image: &str,
    ) -> Option<ResultFrame> {
        let re = OcrImage {
            image: image.to_string(),
        };
        let api = re.ocr_image().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_group_system_msg(
        &mut self,
    ) -> Option<GetGroupSystemMsgResult> {
        let api = GetGroupSystemMsg.get_group_system_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupSystemMsgResult>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn upload_private_file_local(
        &mut self,
        user_id: i64,
        file: &str,
        name: &str,
    ) -> Option<ResultFrame> {
        let re = UploadPrivateFile {
            user_id,
            file: file.to_string(),
            name: name.to_string(),
        };
        let api = re.upload_private_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn upload_private_file_header(
        &mut self,
        user_id: i64,
        file: &str,
        name: &str,
        headers: Vec<String>,
    ) -> Option<ResultFrame> {
        let file = self.download_file(file, headers).await;
        let url = match file {
            None => "".to_string(),
            Some(file) => {
                let value = file.data.unwrap();
                value["file"].to_string()
            }
        };
        let re = UploadPrivateFile {
            user_id,
            file: url,
            name: name.to_string(),
        };

        let api = re.upload_private_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn upload_private_file(
        &mut self,
        user_id: i64,
        file: &str,
        name: &str,
    ) -> Option<ResultFrame> {
        let file = self.download_file(file, vec![]).await;
        let url = match file {
            None => "".to_string(),
            Some(file) => {
                let value = file.data.unwrap();
                value["file"].to_string()
            }
        };
        let re = UploadPrivateFile {
            user_id,
            file: url,
            name: name.to_string(),
        };

        let api = re.upload_private_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn upload_group_file_local(
        &mut self,
        group_id: i64,
        file: &str,
        name: &str,
        folder: &str,
    ) -> Option<ResultFrame> {
        let re = UploadGroupFile {
            group_id,
            file: file.to_string(),
            name: name.to_string(),
            folder: folder.to_string(),
        };
        let api = re.upload_group_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn upload_group_file_header(
        &mut self,
        group_id: i64,
        file: &str,
        name: &str,
        folder: &str,
        headers: Vec<String>,
    ) -> Option<ResultFrame> {
        let file = self.download_file(file, headers).await;
        let url = match file {
            None => "".to_string(),
            Some(file) => {
                let value = file.data.unwrap();
                value["file"].to_string()
            }
        };
        let re = UploadGroupFile {
            group_id,
            file: url,
            name: name.to_string(),
            folder: folder.to_string(),
        };

        let api = re.upload_group_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn upload_group_file(
        &mut self,
        group_id: i64,
        file: &str,
        name: &str,
        folder: &str,
    ) -> Option<ResultFrame> {
        let file = self.download_file(file, vec![]).await;
        let url = match file {
            None => "".to_string(),
            Some(file) => {
                let value = file.data.unwrap();
                value["file"].to_string()
            }
        };
        let re = UploadGroupFile {
            group_id,
            file: url,
            name: name.to_string(),
            folder: folder.to_string(),
        };

        let api = re.upload_group_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }

    pub async fn download_file(
        &mut self,
        url: &str,
        headers: Vec<String>,
    ) -> Option<ResultFrame> {
        let re = DownloadFile {
            url: url.to_string(),
            thread_count: 8,
            headers,
        };
        let api = re.download_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_group_file_system_info(
        &mut self,
        group_id: i64,
    ) -> Option<GetGroupFileSystemInfoRequest> {
        let re = GetGroupFileSystemInfo {
            group_id,
        };
        let api = re.get_group_file_system_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupFileSystemInfoRequest>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_group_root_files(
        &mut self,
        group_id: i64,
    ) -> Option<GetGroupRootFilesRequest> {
        let re = GetGroupRootFiles {
            group_id,
        };
        let api = re.get_group_root_files().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupRootFilesRequest>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_group_files_by_folder(
        &mut self,
        group_id: i64,
        folder_id: &str,
    ) -> Option<GetGroupFilesByFolderRequest> {
        let re = GetGroupFilesByFolder {
            group_id,
            folder_id: folder_id.to_string(),
        };
        let api = re.get_group_files_by_folder().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupFilesByFolderRequest>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn create_group_file_folder(
        &mut self,
        group_id: i64,
        name: &str,
        parent_id: &str,
    ) -> Option<ResultFrame> {
        let re = CreateGroupFileFolder {
            group_id,
            name: name.to_string(),
            parent_id: parent_id.to_string(),
        };
        let api = re.create_group_file_folder().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn delete_group_folder(
        &mut self,
        group_id: i64,
        folder_id: &str,
    ) -> Option<ResultFrame> {
        let re = DeleteGroupFolder {
            group_id,
            folder_id: folder_id.to_string(),
        };
        let api = re.delete_group_folder().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn delete_group_file(
        &mut self,
        group_id: i64,
        file_id: &str,
        busid: i32,
    ) -> Option<ResultFrame> {
        let re = DeleteGroupFile {
            group_id,
            file_id: file_id.to_string(),
            busid,
        };
        let api = re.delete_group_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_group_file_url(
        &mut self,
        group_id: i64,
        file_id: &str,
        busid: i32,
    ) -> Option<GetGroupFileUrlRequest> {
        let re = GetGroupFileUrl {
            group_id,
            file_id: file_id.to_string(),
            busid,
        };
        let api = re.get_group_file_url().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupFileUrlRequest>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_status(
        &mut self,
    ) -> Option<GetStatusRequest> {
        let api = GetStatus.get_status().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetStatusRequest>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_group_at_all_remain(
        &mut self,
        group_id: i64,
    ) -> Option<GetGroupAtAllRemainRequest> {
        let re = GetGroupAtAllRemain {
            group_id,
        };
        let api = re.get_group_at_all_remain().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupAtAllRemainRequest>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn handle_quick_operation(
        &mut self,
        context: Value,
        operation: Value,
    ) -> Option<ResultFrame> {
        let re = HandleQuickOperation {
            context,
            operation,
        };
        let api = re.handle_quick_operation().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn _send_group_notice(
        &mut self,
        group_id: i64,
        content: &str,
        image: &str,
    ) -> Option<ResultFrame> {
        let re = SendGroupNotice {
            group_id,
            content: content.to_string(),
            image: image.to_string(),
        };
        let api = re._send_group_notice().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn _get_group_notice(
        &mut self,
        group_id: i64,
    ) -> Option<GetGroupNoticeRequest> {
        let re = GetGroupNotice {
            group_id,
        };
        let api = re._get_group_notice().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupNoticeRequest>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn reload_event_filter(
        &mut self,
        file: &str,
    ) -> Option<ResultFrame> {
        let re = ReloadEventFilter {
            file: file.to_string(),
        };
        let api = re.reload_event_filter().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_online_clients(
        &mut self,
    ) -> Option<GetOnlineClientsRequest> {
        let re = GetOnlineClients {
            no_cache: false
        };
        let api = re.get_online_clients().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetOnlineClientsRequest>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn get_group_msg_history(
        &mut self,
        group_id: i64,
        message_seq: i64,
    ) -> Option<GetGroupMsgHistoryRequest> {
        let re = GetGroupMsgHistory {
            message_seq,
            group_id,
        };
        let api = re.get_group_msg_history().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetGroupMsgHistoryRequest>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn set_essence_msg(
        &mut self,
        message_id: i64,
    ) -> Option<ResultFrame> {
        let re = SetEssenceMsg {
            message_id,
        };
        let api = re.set_essence_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn delete_essence_msg(
        &mut self,
        message_id: i64,
    ) -> Option<ResultFrame> {
        let re = DeleteEssenceMsg {
            message_id,
        };
        let api = re.delete_essence_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn get_essence_msg_list(
        &mut self,
        group_id: i64,
    ) -> Option<Vec<GetEssenceMsgListRequest>> {
        let re = GetEssenceMsgList {
            group_id,
        };
        let api = re.get_essence_msg_list().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<Vec<GetEssenceMsgListRequest>>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn check_url_safely(
        &mut self,
        url: &str,
    ) -> Option<ResultFrame> {
        let re = CheckUrlSafely {
            url: url.to_string(),
        };
        let api = re.check_url_safely().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn _get_model_show(
        &mut self,
        model: &str,
    ) -> Option<GetModelShowRequest> {
        let re = GetModelShow {
            model: model.to_string(),
        };
        let api = re._get_model_show().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            let result = serde_json::from_value::<GetModelShowRequest>(frame.data.unwrap());
            match result {
                Ok(frame) => Some(frame),
                Err(_) => None
            }
        } else {
            None
        }
    }
    pub async fn _set_model_show(
        &mut self,
        model: &str,
        model_show: &str,
    ) -> Option<ResultFrame> {
        let re = SetModelShow {
            model: model.to_string(),
            model_show: model_show.to_string(),
        };
        let api = re._set_model_show().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn delete_unidirectional_friend(
        &mut self,
        user_id: i64,
    ) -> Option<ResultFrame> {
        let re = DeleteUnidirectionalFriend {
            user_id,
        };
        let api = re.delete_unidirectional_friend().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
    pub async fn send_private_forward_msg(
        &mut self,
        user_id: i64,
        message: Message,
    ) -> Option<ResultFrame> {
        let re = SendPrivateForwardMsg {
            user_id,
            message,
        };
        let api = re.send_private_forward_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        if let Some(frame) = resp {
            Some(frame)
        } else {
            None
        }
    }
}
