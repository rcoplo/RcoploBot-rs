use log::info;
use crate::core::api::GetStrangerInfoResult;
use crate::core::bot::{Bot, ResultFrame};
use crate::core::event::{Event, GroupMemberDecrease, GroupMemberIncrease};
use crate::core::message::{at, text};
use crate::core::notice::Notice;
use crate::core::request::{GroupAddSubType, Request, RequestType};
use crate::bot::{log_result, log_result_by_return, meow_err};
use crate::service::CONTEXT;

//同意 申请入群请求
pub async fn group_handle_module(request: &mut Request) {
    if let RequestType::Group = request.request_type {
        info!("group_handle_module");
        let result = request.set_group_add_request(GroupAddSubType::Add, true, "").await;
        log_result(result);
        let result1 = request.set_group_add_request(GroupAddSubType::Invite, true, "").await;
        log_result(result1);
    }
}
//同意添加好友请求
pub async fn friend_handle_module(request: &mut Request) {
    if let RequestType::Friend = request.request_type {
        let result = request.set_friend_add_request(true, "").await;
        log_result(result);
    }

}

//群成员变动输出消息
pub async fn group_change_handle(event: &Event, bot: &mut Bot){
    match event {
        Event::GroupMemberDecrease(event) => {
            member_change_decrease_handle_module(&mut Notice::new_group_member_decrease(event, bot)).await;
        }
        Event::GroupMemberIncrease(event) => {

            member_change_increase_handle_module(&mut Notice::new_group_member_increase(event, bot)).await;
        }
        _ => {}
    }
}

//群成员增加
pub async fn member_change_increase_handle_module(increase: &mut Notice<GroupMemberIncrease>) {
    if increase.event.user_id != CONTEXT.bot_config.bot_id.unwrap_or(0){
        let result = increase.send_group_msg(vec![at(&increase.event.user_id), text(" 欢迎大佬入群~~")]).await;
        log_result(result);
    }
}

//群成员减少
pub async fn member_change_decrease_handle_module(decrease: &mut Notice<GroupMemberDecrease>) {
    let stranger = decrease.get_stranger_info().await;
    let stranger_operator = decrease.get_stranger_info_user(&decrease.event.operator_id).await;
    match stranger {
        None => {}
        Some(stranger) => {
            if decrease.sub_type == "leave"{
                let result = decrease.send_group_msg(vec![text(&stranger.nickname), text(" 离开了我们...")]).await;
                log_result(result);
            }
            if decrease.sub_type == "kick"{
                let result = decrease.send_group_msg(vec![text(&stranger.nickname), text(format!("被{} 踢了...",stranger_operator.unwrap().nickname))]).await;
                log_result(result);
            }
        }
    }

}