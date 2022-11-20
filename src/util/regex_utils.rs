use log::info;
use serde_json::Value;
use regex::{Regex, RegexSet};
use crate::core::Cq;
use crate::core::message::{at, face};
use crate::service::CONTEXT;


pub fn contain(sources: &String, value: &Vec<&str>) -> bool {
    let bot_name = CONTEXT.bot_config.bot_name.as_ref().unwrap();
    let bot_id = CONTEXT.bot_config.bot_id.as_ref().unwrap();
    let mut vec1 = vec![];
    for msg in value {
        let string = msg.replace("{name}", &bot_name).replace("{id}", bot_id.to_string().as_str());
        vec1.push(format!("^{}$", string));
    }
    let set = RegexSet::new(&vec1).unwrap();
    set.is_match(sources.as_str())
}

pub fn contain_(sources: &String, value: &Vec<&str>) -> bool {
    return if contain(sources, value) {
        false
    } else {
        true
    };
}

pub fn contain_int(sources: &i64, value: &Vec<i64>) -> bool {
    let mut vec1 = vec![];
    for msg in value {
        vec1.push(format!("{}", msg));
    }
    let set = RegexSet::new(&vec1).unwrap();
    set.is_match(sources.to_string().as_str())
}

pub fn contain_list(sources: &str, value: Vec<Vec<&str>>) -> bool {
    for v in value {
        let set = RegexSet::new(&v).unwrap();
        return set.is_match(sources);
    }
    false
}

pub fn replace_regex_all(regex: &str, rep: &str, value: String) -> Value {
    let set = Regex::new(&regex).unwrap();
    let s2 = set.replace_all(&value, rep);
    Value::from(s2.to_string())
}

pub fn replace_regex(regex: &str, rep: &str, value: String) -> Value {
    let set = Regex::new(&regex).unwrap();
    let s2 = set.replace(&value, rep);
    Value::from(s2.to_string())
}


//匹配类型
pub fn rex_utils(r#type: i8, msg: Vec<String>, name: Option<String>, message: &str, qq: Option<i64>) -> (bool, String) {
    let bot_name = CONTEXT.bot_config.bot_name.as_ref().unwrap().clone();
    let bot_id = CONTEXT.bot_config.bot_id.as_ref().unwrap();
    let string = name.unwrap_or(bot_name);
    let mut string1 = String::new();
    for x in &msg {
        string1.push_str(x.as_str());
    }
    let data = match r#type {
        1 => {
            if msg.len() == 1 {
                let name_msg = format!("{}{}", string.clone(), message);
                let msg_name = format!("{}{}", message, string.clone());
                let yes = RegexSet::new(&vec![name_msg, msg_name]).unwrap();
                return (yes.is_match(string1.as_str()), msg[0].to_string());
            } else {
                (false, "".to_string())
            }
        }
        2 => {
            if msg.len() == 2 && msg[0].contains("at") {
                let at = format!("at,{}[\\s]*{}", qq.unwrap_or(*bot_id), message);
                let yes = RegexSet::new(&vec![at]).unwrap();
                return (yes.is_match(string1.as_str()), msg[1].to_string());
            } else {
                (false, "".to_string())
            }
        }
        3 => {
            if msg.len() > 2 && msg[0].contains("reply") {
                let reply = format!("reply,(.*),{0},(.*)at,{0}(.*)[\\s]*(at,{0}(.*))?[\\s]*{1}", qq.unwrap_or(*bot_id), message);
                info!("{}",&reply);
                info!("{}",&string1);
                let yes = RegexSet::new(&vec![reply]).unwrap();
                return (yes.is_match(string1.as_str()), msg[2].to_string());
            } else {
                (false, "".to_string())
            }
        }
        4 => {
            if msg.len() == 1 || msg.len() == 2 && msg[0].contains("at") {
                let name_msg = format!("{}{}", string.clone(), message);
                let msg_name = format!("{}{}", message, string.clone());
                let at = format!("at,{}[\\s]*{}", qq.unwrap_or(*bot_id), message);
                let yes = RegexSet::new(&vec![at, name_msg, msg_name]).unwrap();
                return (yes.is_match(string1.as_str()), string1.replace(" ", ""));
            } else {
                (false, "".to_string())
            }
        }
        _ => (false, "".to_string())
    };
    data
}
