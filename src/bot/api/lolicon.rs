use std::collections::HashMap;
use std::iter::Map;
use log::info;
use rbatis::rbdc::datetime::FastDateTime;
use reqwest::Client;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use crate::domain::Setu;

const URL:&str = "https://api.lolicon.app/setu/v2";
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Lolicon{
    pub aiType:i8,
    pub author:String,
    pub ext:String,
    pub height:i32,
    pub p:i8,
    pub pid:i64,
    pub r18:bool,
    pub tags:Vec<String>,
    pub title:String,
    pub uid:i64,
    pub uploadDate:i64,
    pub urls:Original,
    pub width:i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Original{
    pub url:String,

}

async fn get(url: &str, data:Value) -> Option<Value>{
   let client: Client = reqwest::Client::new();
    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

     let resp = client.post(URL).headers(headers).json(&data).send().await.ok()?;
    let result:Value = serde_json::from_str(resp.text().await.unwrap().as_str()).unwrap();
     Some(result)
}

pub async fn get_lolicon_list_r18(num:i8)-> Option<Vec<Setu>>{
   let json =  json!(
       {
            "r18": 1,
            "num": num,
        }
    );

    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            Some(to_setu_list(data).await)
        }
    }

}

pub async fn get_lolicon_list(num:i64)-> Option<Vec<Setu>>{
    let json =  json!(
       {
            "r18": 0,
            "num": num,
        }
    );

    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            Some(to_setu_list(data).await)
        }
    }
}
pub async fn get_lolicon_list_tag(num:i64,tag:Vec<String>)-> Option<Vec<Setu>>{
    let value = Value::from(tag);
    let json =  json!(
       {
            "r18": 0,
            "num": num,
            "tag": value,
        }
    );

    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            Some(to_setu_list(data).await)
        }
    }
}

pub async fn get_lolicon_r18_tag(tag:Vec<String>)-> Option<Setu>{
    let value = Value::from(tag);
    let json =  json!(
       {
            "r18": 1,
            "tag": value,
        }
    );

    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu(data).await
        }
    }
}
pub async fn get_lolicon_r18()-> Option<Setu>{
    let json =  json!(
       {
            "r18": 1,
        }
    );
    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu(data).await
        }
    }
}
pub async fn get_lolicon()-> Option<Setu>{
    let json =  json!(
       {
            "r18": 0,
        }
    );
    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu(data).await
        }
    }
}

pub async fn get_lolicon_tag(tag:Vec<String>)-> Option<Setu>{
    let value = Value::from(tag);
    let json =  json!(
       {
            "r18": 0,
            "tag": value,
        }
    );
    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu(data).await
        }
    }
}

async fn to_setu(data:Value) -> Option<Setu>{
    let url = data["data"][0]["urls"].as_object().unwrap();
    let vec1 = data["data"][0]["tags"].as_array().unwrap().clone();
    let mut string = String::new();
    for x in vec1 {
        string.push_str(x.as_str().unwrap());
    }
    Some(Setu{
        id: 0,
        title: Some(data["data"][0]["title"].as_str().unwrap().to_string()),
        author: Some(data["data"][0]["author"].as_str().unwrap().to_string()),
        ext: Some(data["data"][0]["ext"].as_str().unwrap().to_string()),
        uid: Some(data["data"][0]["uid"].as_i64().unwrap() ),
        pid: Some(data["data"][0]["pid"].as_i64().unwrap()),
        tags: Some(string),
        r18: Some(data["data"][0]["r18"].as_bool().unwrap() ),
        width: Some(data["data"][0]["width"].as_i64().unwrap() as i32),
        height: Some(data["data"][0]["height"].as_i64().unwrap() as i32),
        urls: Some(url.get("original")
            .unwrap()
            .as_str().unwrap().to_string()),
        path: Some("".to_string()),
        upload_date: Some(FastDateTime::from_timestamp_millis(data["data"][0]["uploadDate"].as_i64().unwrap())),
    })
}
async fn to_setu_list(data:Value) -> Vec<Setu>{
    let mut vec = Vec::new();

    for data in data["data"].as_array().unwrap() {
        let url = data["urls"].as_object().unwrap();
        let vec1 = data["tags"].as_array().unwrap().clone();
        let mut string = String::new();
        for x in vec1 {
            string.push_str(x.as_str().unwrap());
        }
        let setu = Setu {
            id: 0,
            title: Some(data["title"].as_str().unwrap().to_string()),
            author: Some(data["author"].as_str().unwrap().to_string()),
            ext: Some(data["ext"].as_str().unwrap().to_string()),
            uid: Some(data["uid"].as_i64().unwrap() ),
            pid: Some(data["pid"].as_i64().unwrap()),
            tags: Some(string),
            r18: Some(data["r18"].as_bool().unwrap() ),
            width: Some(data["width"].as_i64().unwrap() as i32),
            height: Some(data["height"].as_i64().unwrap() as i32),
            urls: Some(url.get("original")
                .unwrap()
                .as_str().unwrap().to_string()),
            path: Some("".to_string()),
            upload_date: Some(FastDateTime::from_timestamp_millis(data["uploadDate"].as_i64().unwrap())),
        };
        vec.push(setu);
    }
    vec

}