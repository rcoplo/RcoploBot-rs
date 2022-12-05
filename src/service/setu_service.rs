use futures_util::StreamExt;
use rand::Rng;
use rbatis::rbdc::Error;
use regex::internal::Input;

use crate::domain::{ Setu};
use crate::pool;

pub struct SetuService{}

impl SetuService {
    pub async fn query_setu_pid(pid: i64) -> Option<Setu> {
        let setu = Setu::select_by_pid(pool!(), pid).await;
        match setu {
            Ok(setu) => setu,
            Err(_) => None
        }
    }
    pub async fn rand_setu() -> Option<Setu> {
        let i = Self::query_setu_list().await.unwrap();
        let rand = rand::thread_rng().gen_range(0..i);
        let setu = Setu::select_by_id(pool!(), rand).await;
        match setu {
            Ok(setu) => setu,
            Err(_) => None
        }
    }
     async fn query_setu_list() -> Option<usize> {
         let list = Setu::select_all(pool!()).await.unwrap();
         Some(list.len())
    }

}
