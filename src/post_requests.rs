use std::collections::HashMap;
use std::fs;


use crate::router::TargetFunction;

use crate::requests::{self, Request};
use crate::response::{Response, StatusCode};
use crate::router::Router;
use crate::database::{self, DataBase};


pub fn create_post_request_hashmap() -> HashMap<String,TargetFunction>{
    let mut hash_map: HashMap<String, TargetFunction> = HashMap::new();
    hash_map.insert("/msg".to_string(), post_msg);
    return hash_map;
}

fn post_msg(request: &Request,database: &mut DataBase) -> Response{
    let json_data = &request.body;
    database.add_data_by_json(json_data);
    Response::ok(HashMap::new(), "".to_string())
}
