use std::array::IntoIter;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::core::component::event::PostType::Null;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub r#type: String,
    pub data: HashMap<String, String>,

}

pub fn text<T:  AsRef<str>>(text: T) -> Message {
    Message {
        r#type: "text".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("text".to_string(),text.as_ref().to_string()),
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

pub fn at(qq: &i64) -> Message {
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

pub fn at_name(qq: &i64, name: &str) -> Message {
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

pub fn music(r#type: &str, id: &i64) -> Message {
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

pub fn image<T:  AsRef<str>>(url: T) -> Message {
    Message {
        r#type: "image".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("file".to_string(), url.as_ref().to_string()),
        ])),
    }
}

pub fn reply(message_id: &i64) -> Message {
    Message {
        r#type: "reply".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("id".to_string(), message_id.to_string()),
        ])),
    }
}

pub fn reply_text(qq: &i64, text: &str) -> Message {
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

pub fn poke(qq: &i64) -> Message {
    Message {
        r#type: "poke".to_string(),
        data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
            ("qq".to_string(), qq.to_string()),
        ])),
    }
}

pub fn gift(qq: &i64, id: i32) -> Message {
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

pub fn forward_node(name: &str, uin: &i64, content: Vec<Message>) -> Message {
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

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.r#type.as_str() {
            "text" => {
                write!(f,"{}", self.data.get("text").unwrap_or(&"null".to_string()))
            }
            "face" => {
                write!(f,"face{{{}}}",
                        self.data.get("face").unwrap_or(&"null".to_string()),)
            }
            "record" => {
                write!(f,"record{{{},{}}}",
                        self.data.get("file").unwrap_or(&"null".to_string()),
                        self.data.get("url").unwrap_or(&"null".to_string()))
            }
            "video" => {
                write!(f,"video{{{}}}", self.data.get("file").unwrap_or(&"null".to_string()),)
            }
            "at" => {
                write!(f,"at{{{},{}}}",
                        self.data.get("qq").unwrap_or(&"null".to_string()),
                        self.data.get("name").unwrap_or(&"null".to_string()))
            }
            "share" => {
                write!(f,"share{{{},{},{},{}}}",
                        self.data.get("url").unwrap_or(&"null".to_string()),
                        self.data.get("title").unwrap_or(&"null".to_string()),
                        self.data.get("content").unwrap_or(&"null".to_string()),
                        self.data.get("image").unwrap_or(&"null".to_string()),
                )
            }
            "image" => {
                write!(f,"image{{{},{},{}}}",
                        self.data.get("url").unwrap_or(&"null".to_string()),
                        self.data.get("file").unwrap_or(&"null".to_string()),
                        self.data.get("subType").unwrap_or(&"null".to_string()),
                )
            }
            "reply" => {
                write!(f,"reply{{{},{},{}}}",
                        self.data.get("id").unwrap_or(&"null".to_string()),
                        self.data.get("qq").unwrap_or(&"null".to_string()),
                        self.data.get("text").unwrap_or(&"null".to_string()),
                )
            }
            "redbag" => {
                write!(f,"redbag{{{}}}",
                        self.data.get("title").unwrap_or(&"null".to_string()),
                )
            }
            "forward" => {
                write!(f,"forward{{{}}}",
                        self.data.get("id").unwrap_or(&"null".to_string()),
                )
            }
            "xml" => {
                write!(f,"xml{{{},{}}}",
                        self.data.get("data").unwrap_or(&"null".to_string()),
                        self.data.get("resid").unwrap_or(&"null".to_string()),
                )
            }
            "json" => {
                write!(f,"json{{{},{}}}",
                        self.data.get("data").unwrap_or(&"null".to_string()),
                        self.data.get("resid").unwrap_or(&"null".to_string()),
                )
            }
            _ => {
                write!(f,"null")
            }
        }
    }
}

pub fn message_type_handle(message: Vec<Message>) -> String {
    let mut str = String::new();
    for msg in message {
        let data = msg.to_string();
        str.push_str(data.as_str());
    }
    str
}





