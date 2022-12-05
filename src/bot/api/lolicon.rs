use std::collections::HashMap;
use std::iter::Map;
use log::info;

use reqwest::Client;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};

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
    pub urls:Urls,
    pub width:i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Urls{
    pub original:String,

}

async fn get(url: &str, data:Value) -> Option<Value>{
   let client: Client = Client::new();
    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

     let resp = client.post(URL).headers(headers).json(&data).send().await.ok()?;
    let result:Value = serde_json::from_str(resp.text().await.unwrap().as_str()).unwrap();
     Some(result)
}

pub async fn get_lolicon_list_r18(num:i8)-> Option<Vec<Lolicon>>{
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

pub async fn get_lolicon_list(num:i64)-> Option<Vec<Lolicon>>{
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
pub async fn get_lolicon_list_tag(num:i64,tag:Vec<String>)-> Option<Vec<Lolicon>>{
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

pub async fn get_lolicon_r18_tag(tag:Vec<String>)-> Option<Lolicon>{
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
pub async fn get_lolicon_r18()-> Option<Lolicon>{
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
pub async fn get_lolicon()-> Option<Lolicon>{
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

pub async fn get_lolicon_tag(tag:Vec<String>)-> Option<Lolicon>{
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

async fn to_setu(data:Value) -> Option<Lolicon>{
    let data = data["data"].as_array().unwrap()[0].clone();
    let lolicon = serde_json::from_value::<Lolicon>(data).unwrap();
    Some(lolicon)
}

async fn to_setu_list(data:Value) -> Vec<Lolicon>{
    let mut vec = Vec::new();
    for data in data["data"].as_array().unwrap() {
        let lolicon = serde_json::from_value::<Lolicon>(data.clone()).unwrap();
        vec.push(lolicon);
    }
    vec

}