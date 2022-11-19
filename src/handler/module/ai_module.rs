use log::info;
use once_cell::sync::Lazy;
use rand::Rng;
use regex::RegexSet;
use url::form_urlencoded::parse;
use crate::core::group::Group;
use crate::core::message::text;
use crate::handler::{bot_name_compound, log_result};
use crate::service::CONTEXT;
use crate::util::file::get_data_path;
use crate::util::regex_utils::contain;


pub static AI: Lazy<Vec<Ai>> = Lazy::new(|| Ai::new());
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct Ai {
    pub r#type: String,
    pub data: Vec<String>,
}

pub async fn ai_group_module_handle(group: &mut Group) {
    if contain(&group.raw_message,&vec![
        bot_name_compound(0,"(.*)").as_str(),
        bot_name_compound(1,"(.*)").as_str(),
    ]) {
        let ai = &AI.to_vec();
        let name = CONTEXT.bot_config.bot_name.as_ref().unwrap();
        let msg = &group.raw_message.replace(name.as_str(), "");
        let regex_set = RegexSet::new(vec![format!("[{}]", msg)]).unwrap();
        for data in ai {
            if regex_set.is_match(data.r#type.as_str()){
                let i = data.data.len();
                let i1 = rand::thread_rng().gen_range(0..i);
                let result = group.send_group_msg(vec![text(data.data.get(i1).unwrap())]).await;
                log_result(result);
                return;
            }
        }
    }
}

impl Ai {
   pub fn new() -> Vec<Self> {
        let data = include_str!("../../../resources/data/data.json");
        let ai:Vec<Ai> = serde_yaml::from_str(data).expect("[Bot] load config file fail");
        ai
    }
}