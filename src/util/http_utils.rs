use futures_util::TryFutureExt;
use reqwest::header::HeaderMap;
use reqwest::Response;
use serde_json::Value;

pub async fn get_json(url:&str) -> Option<Value>{
    let data = reqwest::get(url).await;
    match data {
        Ok(data) => {
            let result = data.text().await.ok();
            serde_json::from_str(result.unwrap_or_default().as_str()).ok().unwrap_or_default()
        }
        Err(_) => None
    }
}


pub async fn get_resp(url:&str) -> Option<Response>{
    let data = reqwest::get(url).await;
    match data {
        Ok(data) => {
           Some(data)
        }
        Err(_) => None
    }
}

pub async fn post_json(url:&str, header:HeaderMap,body:Value) -> Option<Value>{
    let client = reqwest::Client::new();
    let builder = client.post(url).headers(header).json(&body).send().await;
    match builder {
        Ok(data) => {
            let result = data.text().await.ok();
            serde_json::from_str(result.unwrap_or_default().as_str()).ok().unwrap_or_default()
        }
        Err(_) => None
    }
}
pub async fn post_resp(url:&str, header:HeaderMap,body:Value) -> Option<Response>{
    let client = reqwest::Client::new();
    let builder = client.post(url).headers(header).json(&body).send().await;
    match builder {
        Ok(data) => {
           Some(data)
        }
        Err(_) => None
    }
}


