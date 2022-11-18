use rbatis::rbdc::datetime::FastDateTime;


#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub id:i32,
    pub article_title: Option<String>,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub thumb: Option<String>,
    pub content: Option<String>,
    pub explain: Option<String>,
    pub classification: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub update_time: Option<FastDateTime>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BilibiliSub {
    pub id:i32,
    pub sub_id: Option<i64>,
    pub sub_type: Option<String>,
    pub sub_users: Option<String>,
    pub live_status: Option<i32>,
    pub uid: Option<i64>,
    pub uname: Option<String>,
    pub latest_video_created: Option<i64>,
    pub dynamic_upload_time: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Bot {
    pub bot_id:i32,
    pub bot_name: Option<String>,
    pub bot_ws_url: Option<String>,
    pub bot_token: Option<String>,
    pub bot_qq: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct File {
    pub id:i32,
    pub file_title: Option<String>,
    pub file_type: Option<String>,
    pub file_urls: Option<String>,
    pub add_time: Option<FastDateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GroupFunction {
    pub id:i32,
    pub group_id: Option<i64>,
    pub function_list: Option<String>,
    pub modify_time: Option<FastDateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Image {
    pub id:i32,
    pub image_title: Option<String>,
    pub image_note: Option<String>,
    pub image_urls: Option<String>,
    pub image_thumb: Option<String>,
    pub add_time: Option<FastDateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct McMcsm {
    pub id:i32,
    pub group_id: Option<i64>,
    pub apikey: Option<String>,
    pub server_name: Option<String>,
    pub logo: Option<String>,
    pub daemon_id: Option<String>,
    pub abbreviation: Option<String>,
    pub server_id: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub update_time: Option<FastDateTime>,
    pub create_by: Option<FastDateTime>,
    pub update_by: Option<FastDateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct McServer {
    pub id:i32,
    pub group_id: Option<i64>,
    pub abbreviation: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct McVote {
    pub id:i32,
    pub qq: Option<i64>,
    pub qq_user_name: Option<String>,
    pub package_name: Option<String>,
    pub vote_date: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct McWhite {
    pub id:i32,
    pub abbreviation: Option<String>,
    pub mc_name: Option<String>,
    pub mc_uuid: Option<String>,
    pub user_id: Option<i64>,
    pub add_time: Option<FastDateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Osu {
    pub id:i32,
    pub osu_id: Option<i64>,
    pub osu_name: Option<String>,
    pub osu_mode: Option<String>,
    pub osu_mode_type: Option<i8>,
    pub qq_number: Option<i64>,
    pub create_time: Option<FastDateTime>,
    pub update_time: Option<FastDateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OsuSb {
    pub id:i32,
    pub osu_id: Option<i64>,
    pub osu_name: Option<String>,
    pub osu_mode: Option<String>,
    pub osu_mode_type: Option<i8>,
    pub qq_number: Option<i64>,
    pub create_time: Option<FastDateTime>,
    pub update_time: Option<FastDateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QuestionAnswer {
    pub id:i32,
    pub user_id: Option<i64>,
    pub group_id: Option<i64>,
    pub question: Option<String>,
    pub answer: Option<String>,
    pub percentage: Option<i32>,
    pub default_answer: Option<i32>,
    pub is_random: Option<i8>,
    pub create_time: Option<FastDateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Setu {
    pub id: i32,
    pub title: Option<String>,
    pub author: Option<String>,
    pub ext: Option<String>,
    pub uid: Option<i64>,
    pub pid: Option<i64>,
    pub tags: Option<String>,
    pub r18: Option<bool>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub urls: Option<String>,
    pub path: Option<String>,
    pub upload_date: Option<FastDateTime>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SignGroupUsers {
    pub id:i32,
    pub user_id: Option<i64>,
    pub group_id: Option<i64>,
    pub checkin_count: Option<i32>,
    pub checkin_count_last: Option<FastDateTime>,
    pub impression: Option<f64>,
    pub impression_grade: Option<i32>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UsersFunction {
    pub id:i32,
    pub user_id: Option<i64>,
    pub function_list: Option<String>,
    pub modify_time: Option<FastDateTime>,
}
