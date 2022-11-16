use serde_json::{Map, Value};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseApi<P>{
    pub action: String,
    pub params: P,
    pub echo: String,
}

impl<P> BaseApi<P> {
    pub fn new(action: String, params: P) -> Option<BaseApi<P>> {
        let api = BaseApi {
            action,
            params,
            echo:"".to_string(),
        };
        Some(api)
    }
}


