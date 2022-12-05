
use std::fs;
use std::path::PathBuf;
use log::info;

fn get_file_resources_path(file:Vec<&str>) -> String{
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources");
    for file in file {
        path.push(file);
    }
    return path.to_str().unwrap().to_string()
}

pub fn get_data_path(file:Vec<&str>) -> String{
    let mut v = vec!["data"];
    return if file.len() == 0 {
        get_file_resources_path(v)
    } else {
        for file in file {
            v.push(file);
        }
        get_file_resources_path(v)
    }
}

pub fn get_image_path(file:Vec<&str>) -> String{
    let mut v = vec!["image"];
    return if file.len() == 0 {
        get_file_resources_path(v)
    } else {
        for file in file {
            v.push(file);
        }
        get_file_resources_path(v)
    }
}

pub fn get_font_path(file:Vec<&str>) -> String{
    let mut v = vec!["font"];
    return if file.len() == 0 {
        get_file_resources_path(v)
    } else {
        for file in file {
            v.push(file);
        }
        get_file_resources_path(v)
    }
}

pub fn get_tmp_path(file:Vec<&str>) -> String{
    let mut v = vec!["tmp"];
    return if file.len() == 0 {
        get_file_resources_path(v)
    } else {
        for file in file {
            v.push(file);
        }
        get_file_resources_path(v)
    }
}

pub fn tmp_random_image_path(name:&str,ext:&str,file:Vec<&str>) ->String{
    let file = get_tmp_path(file);
    let string = uuid::Uuid::new_v4().to_string();
    let string1 = string.replace("-", "_");
    let mut path = format!("{}/{}", file, string1);
    path.push_str(format!("_{}.{}",name,ext).as_str());
    path
}
