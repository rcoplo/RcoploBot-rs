use serde_json::Value;
use regex::{Regex, RegexSet};


pub fn contain(sources: &String, value: Vec<&str>) -> bool {
    let set = RegexSet::new(&value).unwrap();
    set.is_match(sources.as_str())
}

pub fn contain_list(sources: &str, value: Vec<Vec<&str>>) -> bool {
    for v in value {
        let set = RegexSet::new(&v).unwrap();
        return set.is_match(sources);
    }
    false
}

pub fn replace_regex_all(regex:&str, rep:&str,value: String) -> Value {
    let set = Regex::new(&regex).unwrap();
    let s2 = set.replace_all(&value, rep);
    Value::from(s2.to_string())
}

pub fn replace_regex(regex:&str, rep:&str,value: String) -> Value {
    let set = Regex::new(&regex).unwrap();
    let s2 = set.replace(&value, rep);
    Value::from(s2.to_string())
}