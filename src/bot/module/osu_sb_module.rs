use std::collections::HashMap;
use reqwest::Error;
use crate::core::bot::Bot;
use crate::core::event::Event;
use crate::core::group::Group;
use crate::core::message::text;
use crate::bot::api::osu_sb::{get_sou_sb_scores, OsuSbScores};
use crate::bot::bot_help::{BOT_HELP, BotHelp, Help};
use crate::bot::log_result;
use crate::util::regex_utils::contain;

pub struct OsuSbHelp;
impl BotHelp for OsuSbHelp{
    fn new() -> Help<'static> {
        Help{
            module_name: "".to_string(),
            module_name_abbreviation: "".to_string(),
            module_order: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("pr",vec!["/pr"]),
                ("pr_@",vec!["/pr[\\s]+at,[0-9]*,null(.*)"]),
                ("pr_name",vec!["/pr[\\s]+\\w+"]),
            ])),
            module_default: true,
            module_help: vec![]
        }
    }
}

pub async fn osu_sb_group_module_handle(group:&mut Group) {
    let osusb_help = BOT_HELP.help.get("osusb").unwrap();
    if contain(&group.message_list[0],osusb_help.module_order.get("pr").unwrap()) {
        pr(group).await;
    }
}




pub async fn pr(group:&mut Group){
    let sou_sb_scores = get_sou_sb_scores(4149, 3, "recent", 1).await;
    match sou_sb_scores {
        Ok(scores) => {
            let vec1 = vec![text(scores.player.name)];
            let result = group.send_group_msg(vec1).await;
            log_result(result);
        }
        Err(_) => {}
    }
}