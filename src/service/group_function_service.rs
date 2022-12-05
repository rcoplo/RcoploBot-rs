use std::collections::HashMap;
use log::{error, info};
use once_cell::sync::Lazy;
use rbatis::rbdc::Error;
use serde_json::Value;
use crate::domain::GroupFunction;
use crate::pool;

pub struct GroupFunctionService;


impl GroupFunctionService {
    //查询模块权限
    pub async fn select_function(group_id:&i64) -> Option<GroupFunction>{
        let function = GroupFunction::select_group_function(pool!(), group_id).await;
        match function {
            Ok(data) => {
                match data {
                    None => None,
                    Some(data) => {
                        Some(data)
                    }
                }
            }
            Err(_) => {
                error!("获取群权限数据失败!");
                None
            }
        }
    }
    //是否开启模块功能
    pub async fn select_function_is_open(group_id:&i64,function_name:&str) -> bool{
        let function = GroupFunction::select_group_function(pool!(), group_id).await;
        match function {
            Ok(data) => {
                match data {
                    None => false,
                    Some(data) => {
                        let string = data.function_list.unwrap();
                        let result:Value = serde_json::from_str(string.as_str()).unwrap();
                        let option = result.as_object().unwrap();
                        if option.get(function_name).unwrap().as_bool() == Some(true){
                            true
                        }else {
                            false
                        }
                    }
                }
            }
            Err(_) => {
                error!("获取群权限数据失败!");
                false
            }
        }
    }
    //添加
    pub async fn insert_function(group:GroupFunction){
        let function = GroupFunction::insert(pool!(),&group).await;
        match function {
            Ok(data) => {
                log::info!("添加群权限数据成功!");
            }
            Err(_) => {
                error!("添加群权限数据失败!");
            }
        }
    }
    //开启功能
    pub async fn open_function(group_id:&i64,function_name:&str) -> bool{
        let function = GroupFunctionService::select_function(&group_id).await;
        match function {
            Some(data) => {
                let string = data.function_list.as_ref().unwrap();
                let mut result:Value = serde_json::from_str(string.as_str()).unwrap();
                let option = result
                    .as_object_mut()
                    .unwrap();
                    option.insert(function_name.to_string().clone(),Value::from(true));
                let value = Value::from(option.clone());
                Self::update_function(data,&group_id,value.to_string()).await;
                info!("开启功能 {} 成功!",&function_name);
                true
            }
            None => {
                info!("开启功能 {} 失败!",&function_name);
                false
            }
        }
    }
    //关闭功能
    pub async fn close_function(group_id:&i64,function_name:&str) -> bool{
        let function = GroupFunctionService::select_function(&group_id).await;
        match function {
            Some(data) => {
                let string = data.function_list.as_ref().unwrap();
                let mut result:Value = serde_json::from_str(string.as_str()).unwrap();
                let option = result.as_object_mut().unwrap();
                    option.insert(function_name.to_string().clone(),Value::from(false));
                let value = Value::from(option.clone());
                Self::update_function(data,&group_id,value.to_string()).await;
                info!("关闭功能 {} 成功!",&function_name);
                true
            }
            None => {
                info!("关闭功能 {} 失败!",&function_name);
                false
            }
        }
    }


    pub async fn update_function(group:GroupFunction,group_id:&i64,function_list:String){
        let function1 = GroupFunction {
            function_list: Some(function_list),
            ..group
        };
        let function = GroupFunction::update_group_function(pool!(),&function1,group_id).await;
        match function {
            Ok(_) => {
                log::info!("更改群权限列表成功!");
            }
            Err(_) => {
                error!("更改群权限列表失败!");
            }
        }
    }
    pub async fn get_group_function_list() -> HashMap<i64, GroupFunction> {
        let function = GroupFunction::select_group_function_list(pool!()).await;
        match function {
            Ok(data) => {
                let mut map = HashMap::new();
                for x in data {
                    map.insert(x.group_id.unwrap(),x);
                }
                map
            }
            Err(_) => {
                error!("获取群权限数据失败!");
                HashMap::new()
            }
        }
    }
}
