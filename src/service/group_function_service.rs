use log::error;
use rbatis::rbdc::Error;
use serde_json::Value;
use crate::domain::GroupFunction;
use crate::pool;

pub struct GroupFunctionService;


impl GroupFunctionService {
    pub async fn select_function(group_id:i64) -> Option<Value>{
        let function = GroupFunction::select_group_function(pool!(), group_id).await;
        match function {
            Ok(data) => {
                serde_json::to_value(data).ok()
            }
            Err(_) => {
                error!("获取群权限数据失败!");
                None
            }
        }
    }
    pub async fn insert_function(group:GroupFunction) -> bool{
        let function = GroupFunction::insert(pool!(),&group).await;
        match function {
            Ok(data) => {
                true
            }
            Err(_) => {
                error!("获取群权限数据失败!");
                false
            }
        }
    }
}