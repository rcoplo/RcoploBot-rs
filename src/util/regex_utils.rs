use regex::RegexSet;


pub fn contain(sources: &str, value: Vec<&str>) -> bool {
    let set = RegexSet::new(&value).unwrap();
    set.is_match(sources)
}

pub fn contain_list(sources: &str, value: Vec<Vec<&str>>) -> bool {
    for v in value {
        let set = RegexSet::new(&v).unwrap();
        return set.is_match(sources);
    }
    false
}
