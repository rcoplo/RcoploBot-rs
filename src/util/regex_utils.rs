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
        let string = msg
            .replace("{name}", &bot_name)
            .replace("{id}", bot_id.to_string().as_str())
            .replace("{at}", r"at\{\d+,(.+)\} ")
            .replace("{reply}", r"reply\{-?\d+,\d+,(.+)}(.+) ");
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

pub fn replace_regex_all(regex: &str, rep: &str, value: String) -> String {
    let set = Regex::new(&regex).unwrap();
    let s2 = set.replace_all(&value, rep);
    s2.to_string()
}

fn replace_regex(regex: &str, rep: &str, value: String) -> String {
    let set = Regex::new(&regex).unwrap();
    let s2 = set.replace(&value, rep);
    s2.to_string()
}


//匹配类型
pub fn rex_utils(select:i8, msg: &String, name: Option<String>, regex: &str, qq: Option<i64>,no_message:Vec<&str>) -> (bool, String) {
    let bot_name = CONTEXT.bot_config.bot_name.as_ref().unwrap().clone();
    let bot_id = CONTEXT.bot_config.bot_id.as_ref().unwrap();
    let name = name.unwrap_or(bot_name.clone());
    let qq = qq.unwrap_or(*bot_id);

    let vec =  match   select {
        0=> vec![msg_name(regex, &name), name_msg(&name, regex)],
        1=> vec![at_rex(&qq,regex), msg_name(regex, &name), name_msg(&name, regex)],
        2=> vec![at_rex(&qq,regex), reply_rex(&qq,regex), msg_name(regex, &name), name_msg(&name, regex)],
        _ => vec![]
    };
    let result = RegexSet::new(&vec).unwrap();
    let value = replace_regex(r"(.*)}", "", msg.replace("-", ""));
    for x in no_message {
        let set = RegexSet::new(&vec![at_rex(&qq,x), reply_rex(&qq,x), msg_name(x, &name), name_msg(&name, x)]).unwrap();
        if set.is_match(msg.as_str()){
            return (false, value.replace(bot_name.clone().as_str(), ""));
        }
    }
    (result.is_match(msg.as_str()), value.replace(bot_name.clone().as_str(), ""))
}

fn at_rex(qq:&i64,regex:&str) ->String{
    format!(r"^at\{{{0},(.+)}}[\s]*{1}$",qq,regex)
}

fn reply_rex(qq:&i64,regex:&str) ->String{
    format!(r"^reply\{{-?\d+,{0},(.+)}}(.+)[\s]*{1}$",qq,regex)
}

fn msg_name(regex:&str,name:&str) ->String{
    format!("^{}{}$",regex,name)
}

fn name_msg(name:&str,regex:&str) ->String{
    format!("^{}{}$",name,regex)
}