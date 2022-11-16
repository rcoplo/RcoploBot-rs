use crate::core::component::cq;

pub struct Cq;

impl Cq {
    pub fn set(cq:Vec<String>) -> String{
        let mut v = String::new();
        for cq in cq.iter() {
            v.push_str(cq.as_str());
        }
        v
    }

    pub fn image(file: &str) -> String {
        format!("[CQ:image,file={}]", file)
    }

    pub fn text(text: &str) -> String {
        text.to_string()
    }

    pub fn face(id: i32) -> String {
        format!("[CQ:face,id={}]", id)
    }

    pub fn record(file: &str) -> String {
        format!("[CQ:record,file={}]", file)
    }

    pub fn video(file: &str) -> String {
        format!("[CQ:video,file={}]", file)
    }

    pub fn video_all(file: &str, cover: &str) -> String {
        format!("[CQ:video,file={},cover={}]", file, cover)
    }

    pub fn at(qq: i64) -> String {
        format!("[CQ:at,qq={}]", qq)
    }

    pub fn at_name(qq: i64, name: &str) -> String {
        format!("[CQ:at,qq={},name={}]", qq, name)
    }

    pub fn at_all() -> String {
        format!("[CQ:at,qq=all]")
    }

    pub fn share(url: &str, title: &str) -> String {
        format!("[CQ:share,url={},title={}]", url, title)
    }

    pub fn share_all(url: &str, title: &str, content: &str, image: &str) -> String {
        format!("[CQ:share,url={},title={},content={},image={}]", url, title, content, image)
    }

    pub fn music(r#type: &str, id: i64) -> String {
        format!("[CQ:music,type={},id={}]", r#type, id)
    }

    pub fn music_all(r#type: &str, url: &str, audio: &str, title: &str, content: &str, image: &str) -> String {
        format!("[CQ:music,type={},url={},audio={},title={},content={},image={}]", r#type, url, audio, title, content, image)
    }

    pub fn reply(id: i64) -> String {
        format!("[CQ:reply,id={}]", id)
    }

    pub fn reply_all(qq: i64, text: &str) -> String {
        format!("[CQ:reply,text={},qq={}]", text, qq)
    }

    pub fn poke(qq: i64) -> String {
        format!("[CQ:poke,qq={}]", qq)
    }

    pub fn redbag(title: &str) -> String {
        format!("[CQ:redbag,title={}]", title)
    }

}


