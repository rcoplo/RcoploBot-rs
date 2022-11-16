use crate::core::bot::Bot;
use crate::core::event::{Event, GroupMessageEvent};
use crate::util::regex_utils::contain;
use crate::core::message::*;

pub async fn group_recreational_module_handle(event:&Event,bot:&mut Bot){
    let mut bot = bot.clone();
    match event{
        Event::GroupMessageEvent(event) => {
            group_handle(&event, &mut bot).await
        }
        Event::GroupFileUpload(_) => {}
        Event::GroupAdminChange(_) => {}
        Event::GroupMemberDecrease(_) => {}
        Event::GroupMemberIncrease(_) => {}
        Event::GroupBan(_) => {}
        Event::FriendAdd(_) => {}
        Event::GroupMessageRecall(_) => {}
        Event::FriendMessageRecall(_) => {}
        Event::FriendPoke(_) => {}
        Event::GroupPoke(_) => {}
        Event::TipsForLuckyKingOfRedPackets(_) => {}
        Event::GroupMemberHonorChangePrompt(_) => {}
        Event::GroupMemberBusinessCardUpdate(_) => {}
        Event::OfflineFileReceived(_) => {}
        Event::AddFriendRequest(_) => {}
        Event::AddGroupRequest(_) => {}
        Event::OtherClientOnlineStatusChanges(_) => {}
        _ => {}
    }

}


pub async fn group_handle(event:& GroupMessageEvent, bot: &mut Bot) {
    let mut bot = bot.clone();
    if contain(event.raw_message.as_str(), vec!["日群友"]) {
        supermarket_group_member(&event, &mut bot).await;
    };
    if contain(event.raw_message.as_str(), vec!["日群主"]) {
        bot.send_group_msg(event.group_id,vec![at(event.user_id),text(format!("{}", "绒布球不乖了噢"))]).await;
    };
}

async fn supermarket_group_member(event:&GroupMessageEvent, bot: &mut Bot){
    let mut bot = bot.clone();

}


