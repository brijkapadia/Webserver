use std::collections::HashMap;
use std::fs;


use crate::router::TargetFunction;

use crate::requests::{self, Request};
use crate::response::{Response, StatusCode};
use crate::router::Router;
use crate::database::{self, DataBase};


pub fn create_get_request_hashmap() -> HashMap<String,TargetFunction>{
    let mut hash_map: HashMap<String, TargetFunction> = HashMap::new();
    hash_map.insert("/".to_string(), home_page);
    hash_map.insert("/login".to_string(), login_page);
    hash_map.insert("/style.css".to_string(), style);
    hash_map.insert("/index.js".to_string(), index_js);
    hash_map.insert("/data".to_string(), get_data);
    return hash_map;
}

fn create_response_from_body(body: String, content_type: &str) -> Response{
    let mut header: HashMap<String, String> = HashMap::new();
    header.insert("Content-Length".to_string(), body.len().to_string());
    header.insert("Content-Type".to_string(), content_type.to_string());

    Response { version: "HTTP/1.1", status_code: StatusCode::OK,status_msg: "Ok", header, body}
}

fn create_file_response(file_name: &str,content_type: &str) -> Response{
    let body = match fs::read_to_string(file_name){
        Ok(file) =>file,
        Err(e) => return Router::file_not_found()
    };
    create_response_from_body(body,content_type)
}

fn home_page(request: &Request,database: &mut DataBase) -> Response{
    create_file_response("index.html", "text/html")
}

fn login_page(request: &Request,database: &mut DataBase) -> Response{
    create_file_response("login.html", "text/html")
}

fn style(request: &Request,database: &mut DataBase) -> Response{
    create_file_response("style.css", "text/css")
}

fn index_js(request: &Request,database: &mut DataBase) -> Response{
    create_file_response("index.js", "application/javascript")
}

fn get_data(request: &Request,database: &mut DataBase) -> Response{
    let body = database.get_all_data_as_json();
    create_response_from_body(body, "application/json")

}