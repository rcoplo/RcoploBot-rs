use serde_json::Value;
use regex::{Regex, RegexSet};


pub fn contain(sources: &String, value: &Vec<&str>) -> bool {
    let mut vec1 = vec![];
    for msg in value {
        vec1.push(format!("^{}$",msg));
    }
    let set = RegexSet::new(&vec1).unwrap();
    set.is_match(sources.as_str())
}
pub fn contain_int(sources: &i64, value: &Vec<i64>) -> bool {
    let mut vec1 = vec![];
    for msg in value {
        vec1.push(format!("{}",msg));
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