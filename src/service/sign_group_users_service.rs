use log::info;
use rand::Rng;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use crate::domain::SignGroupUsers;
use crate::pool;
use crate::service::CONTEXT;

pub struct SignGroupUsersService;

impl SignGroupUsersService {
    pub async fn select_is_sign(user_id:&i64,group_id:&i64) ->Option<SignGroupUsers>{
        let result = SignGroupUsers::select_by_user_id_group_id(pool!(), &user_id, &group_id).await;
        match result {
            Ok(data) => {
                data
            }
            Err(_) => None
        }
    }
    pub async fn sign(user_id:&i64, group_id:&i64) ->bool {
        let sign = Self::select_is_sign(&user_id, &group_id).await;
        match sign {
            None => false,
            Some(data) => {
                let impression = rand::thread_rng().gen_range(0.0..CONTEXT.config.sign_in.sign_scope);
                let sign = SignGroupUsers {
                    checkin_count: Some(data.checkin_count.unwrap() + 1),
                    checkin_count_last: Some(FastDateTime::now()),
                    impression: Some(data.impression.unwrap() + impression),
                    impression_grade: Some(data.impression_grade.unwrap() + 1),
                    ..data
                };
                let column = SignGroupUsers::update_by_column(pool!(), &sign, "id").await;
                match column {
                    Ok(_) => {
                        info!("修改 sign 数据成功!");
                        true
                    }
                    Err(_) => {info!("修改 sign 数据失败!");
                    false
                    }
                }
            }
        }
    }
    pub async fn insert_sign(user_id:&i64,group_id:&i64) ->bool{
        let sign = SignGroupUsers::new(*user_id,*group_id);
        let result = SignGroupUsers::insert(pool!(), &sign).await;
        match result {
            Ok(data) => {
                true
            }
            Err(_) => false
        }
    }
    pub async fn select_sign_list(group_id:&i64) ->Option<Vec<SignGroupUsers>>{
        // let sign = SignGroupUsers::new(*user_id,*group_id);
        let result = SignGroupUsers::select_sign_list(pool!(), &group_id).await;
        match result {
            Ok(data) => {
                Some(data)
            }
            Err(_) => None
        }
    }
}

impl SignGroupUsers {
    fn new(user_id:i64,group_id:i64) -> Self {
        let impression = rand::thread_rng().gen_range(0.0..CONTEXT.config.sign_in.sign_scope);
        Self{
            id: 0,
            user_id: Some(user_id),
            group_id: Some(group_id),
            checkin_count: Some(1),
            checkin_count_last: Some(FastDateTime::now()),
            impression: Some(impression),
            impression_grade: Some(1)
        }
    }
}