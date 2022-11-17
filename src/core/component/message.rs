use std::array::IntoIter;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::core::component::event::PostType::Null;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub r#type: String,
    pub data: HashMap<String, String>,

}

pub fn text(text: &str) -> Message {
    Message {
        r#type: "text".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("text".to_string(), text.to_string()),
        ])),
    }
}

pub fn face(id: i32) -> Message {
    Message {
        r#type: "face".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("id".to_string(), id.to_string()),
        ])),
    }
}

pub fn record(file: &str) -> Message {
    Message {
        r#type: "record".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("file".to_string(), file.to_string()),
        ])),
    }
}

pub fn video(file: &str) -> Message {
    Message {
        r#type: "video".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("file".to_string(), file.to_string()),
        ])),
    }
}

pub fn at(qq: i64) -> Message {
    Message {
        r#type: "at".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("qq".to_string(), qq.to_string()),
        ])),
    }
}

pub fn at_all() -> Message {
    Message {
        r#type: "at".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("qq".to_string(), "all".to_string()),
        ])),
    }
}

pub fn at_name(qq: i64, name: &str) -> Message {
    Message {
        r#type: "at".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("qq".to_string(), qq.to_string()),
            ("name".to_string(), name.to_string()),
        ])),
    }
}

pub fn share(url: &str, title: &str) -> Message {
    Message {
        r#type: "share".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("url".to_string(), url.to_string()),
            ("title".to_string(), title.to_string()),
        ])),
    }
}

pub fn share_all(url: &str, title: &str, content: &str, image: &str) -> Message {
    Message {
        r#type: "share".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("file".to_string(), url.to_string()),
            ("title".to_string(), title.to_string()),
            ("content".to_string(), content.to_string()),
            ("image".to_string(), image.to_string()),
        ])),
    }
}

pub fn music(r#type: &str, id: i64) -> Message {
    Message {
        r#type: "music".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("type".to_string(), r#type.to_string()),
            ("id".to_string(), id.to_string()),
        ])),
    }
}

pub fn music_all(r#type: &str, url: &str, audio: &str, title: &str, content: &str, image: &str) -> Message {
    Message {
        r#type: "music".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("type".to_string(), r#type.to_string()),
            ("url".to_string(), url.to_string()),
            ("audio".to_string(), audio.to_string()),
            ("title".to_string(), title.to_string()),
            ("content".to_string(), content.to_string()),
            ("image".to_string(), image.to_string()),
        ])),
    }
}

pub fn image(url: &str) -> Message {
    Message {
        r#type: "image".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("file".to_string(), url.to_string()),
        ])),
    }
}

pub fn reply(message_id: i64) -> Message {
    Message {
        r#type: "reply".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("id".to_string(), message_id.to_string()),
        ])),
    }
}

pub fn reply_text(qq: i64, text: &str) -> Message {
    Message {
        r#type: "reply".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("qq".to_string(), qq.to_string()),
            ("text".to_string(), text.to_string()),
        ])),
    }
}

pub fn redbag(title: &str) -> Message {
    Message {
        r#type: "redbag".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("title".to_string(), title.to_string()),
        ])),
    }
}

pub fn poke(qq: i64) -> Message {
    Message {
        r#type: "poke".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("qq".to_string(), qq.to_string()),
        ])),
    }
}

pub fn gift(qq: i64, id: i32) -> Message {
    Message {
        r#type: "gift".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("qq".to_string(), qq.to_string()),
            ("id".to_string(), id.to_string()),
        ])),
    }
}

pub fn forward_ref(id: &str) -> Message {
    Message {
        r#type: "forward".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("id".to_string(), id.to_string()),
        ])),
    }
}

pub fn forward_node(name: &str, uin: i64, content: Vec<Message>) -> Message {
    Message {
        r#type: "node".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("name".to_string(), name.to_string()),
            ("uin".to_string(), uin.to_string()),
            ("content".to_string(), serde_json::to_string(&content).unwrap()),
        ])),
    }
}

pub fn node_ref(message_id: i32) -> Message {
    Message {
        r#type: "node".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("id".to_string(), message_id.to_string()),
        ])),
    }
}


pub fn xml(data: &str) -> Message {
    Message {
        r#type: "xml".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("data".to_string(), data.to_string()),
        ])),
    }
}

pub fn xml_all(data: &str, resid: i32) -> Message {
    Message {
        r#type: "xml".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("data".to_string(), data.to_string()),
            ("resid".to_string(), resid.to_string()),
        ])),
    }
}


pub fn json(data: &str) -> Message {
    let data = data
        .replace(",", "&#44;")
        .replace("&", "&amp;")
        .replace("[", "&#91;")
        .replace("]", "&#93;");
    Message {
        r#type: "json".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("data".to_string(), data.to_string()),
        ])),
    }
}

pub fn json_all(data: &str, resid: i32) -> Message {
    let data = data
        .replace(",", "&#44;")
        .replace("&", "&amp;")
        .replace("[", "&#91;")
        .replace("]", "&#93;");
    Message {
        r#type: "json".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("data".to_string(), data.to_string()),
            ("resid".to_string(), resid.to_string()),
        ])),
    }
}

pub fn cardimage(file: &str) -> Message {
    Message {
        r#type: "cardimage".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("file".to_string(), file.to_string()),
        ])),
    }
}

pub fn cardimage_all(file: &str, minwidth: i32, minheight: i32, maxwidth: i32, maxheight: i32, source: &str, icon: &str) -> Message {
    Message {
        r#type: "cardimage".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("file".to_string(), file.to_string()),
            ("minwidth".to_string(), minwidth.to_string()),
            ("minheight".to_string(), minheight.to_string()),
            ("maxwidth".to_string(), maxwidth.to_string()),
            ("maxheight".to_string(), maxheight.to_string()),
            ("source".to_string(), source.to_string()),
            ("icon".to_string(), icon.to_string()),
        ])),
    }
}

pub fn tts(text: &str) -> Message {
    Message {
        r#type: "tts".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("text".to_string(), text.to_string()),
        ])),
    }
}

pub fn message_type_handle(v: Value) -> Option<HashMap<String,Message>> {
    if v.is_null() { return None; }
    let json = v["message"].as_array();
    let mut map = HashMap::new();
    let msg:Option<HashMap<String,Message>> = match json {
        None => None,
        Some(json) => {
            for value in json.iter() {
                let r#type = value["type"].as_str().unwrap();
                let result = serde_json::from_value::<Message>(value.clone()).unwrap();
                map.insert(r#type.to_string(), result);
            }
            return Some(map);
        }
    };
    msg
}





