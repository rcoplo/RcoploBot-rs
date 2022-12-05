use log::info;
use rbatis::rbdc::datetime::FastDateTime;
use reqwest::Error;
use serde_json::Value;
use serde_json::Value::Null;
use crate::util::http_utils::get_json;

static OSU_SB_URL:&str = "https://api.ppy.sb/";



pub async fn get_sou_sb_scores(id:i32, mode:i32, scope:&str, limit:i32) -> Result<OsuSbScores,reqwest::Error> {
    let json = get_json(format!("{}get_player_scores?id={}&mode={}&scope={}&limit={}", OSU_SB_URL, id, mode, scope, limit)).await;
    match json {
        Ok(json) => {
            info!("{}",json);
            Ok(serde_json::from_value::<OsuSbScores>(json).unwrap())
        }
        Err(err) => Err(err)
    }
}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct OsuSbScores{
    pub status:String,
    pub scores:Vec<Scores>,
    pub player:Player,
}
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct Scores{
    pub id:i32,
    pub score:i32,
    pub pp:f32,
    pub acc:f64,
    pub mods:i32,
    pub n300:i32,
    pub n100:i32,
    pub n50:i32,
    pub nmiss:i32,
    pub ngeki:i32,
    pub nkatu:i32,
    pub grade:char,
    pub status:i32,
    pub mode:i32,
    pub play_time:FastDateTime,
    pub time_elapsed:i32,
    pub perfect:i32,
    pub beatmap:Beatmap,
}
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct Player{
    pub id:i32,
    pub name:String,
    pub clan:Value,

}
#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct Beatmap{
    pub md5:String,
    pub id:i32,
    pub set_id:i32,
    pub artist:String,
    pub title:String,
    pub version:String,
    pub creator:String,
    pub last_update:FastDateTime,
    pub total_length:i32,
    pub max_combo:i32,
    pub status:i32,
    pub plays:i32,
    pub passes:i32,
    pub mode:i32,
    pub bpm:f32,
    pub cs:f32,
    pub od:f32,
    pub ar:f32,
    pub hp:f32,
    pub diff:f64,
}

