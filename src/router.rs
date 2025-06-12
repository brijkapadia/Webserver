use core::hash;
use std::collections::HashMap;
use std::fs;
use crate::requests::{self, Request, RequestError};
use crate::response::{self, Response};
use crate::requests::Method;
use crate::response::StatusCode;
use crate::get_requests::create_get_request_hashmap;
use crate::post_requests::create_post_request_hashmap;
use crate::database::{self, DataBase};

pub type TargetFunction = fn(&Request,&mut DataBase) -> Response;
type hash_map_type = HashMap<String,TargetFunction>;

pub struct Router{
    hash_map_arr: [hash_map_type; 4], //4 is for the number of variants
}

impl Router{
    pub fn new() -> Self{
        let mut hash_map_arr: [hash_map_type; 4] = [HashMap::new(),HashMap::new(),HashMap::new(),HashMap::new()];
        hash_map_arr[0] = create_get_request_hashmap();
        hash_map_arr[1] = create_post_request_hashmap();
        Self {hash_map_arr}
    }

    pub fn add_route(&mut self, method: Method, target: &str, target_function: TargetFunction){
        self.get_method_hash_map_mut(&method).insert(target.to_string(), target_function);
    }

    fn get_method_hash_map_mut(&mut self,method: &Method) -> &mut hash_map_type{
        match method{
            Method::GET => &mut self.hash_map_arr[0],
            Method::POST => &mut self.hash_map_arr[1],
            Method::PUT => &mut self.hash_map_arr[2],
            Method::DELETE => &mut self.hash_map_arr[3],
        }
    }

    fn get_method_hash_map(&self,method: &Method) -> &hash_map_type{
        match method{
            Method::GET => &self.hash_map_arr[0],
            Method::POST => &self.hash_map_arr[1],
            Method::PUT => &self.hash_map_arr[2],
            Method::DELETE => &self.hash_map_arr[3],
        }
    }
    fn create_ok_response(&self, request: &Request,database: &mut DataBase) -> Response{
        if request.version != "HTTP/1.1"{
            return self.create_frame_err_response(StatusCode::HTTPVersionNotSupported, "Expect HTTP/1.1");
        }
        match self.get_method_hash_map(&request.method).get(&request.target){
            Some(target_function) => target_function(request,database),
            None => Router::file_not_found()
        }
    }
    fn create_frame_err_response(&self, status_code: StatusCode, status_msg: &'static str) -> Response{
        response::Response::new_v(status_code, status_msg, HashMap::new(), String::new())
    }
    fn create_err_response(&self,e: &RequestError) -> Response{
        let status_code: StatusCode;
        let status_msg: &str;
        match e{
            RequestError::FormatingError(msg) => self.create_frame_err_response(StatusCode::BadRequest, "Formatting Error"),
            RequestError::MethodNotAllowed => self.create_frame_err_response(StatusCode::MethodNotAllowed, "Method not allowed"),
            RequestError::LengthRequired => self.create_frame_err_response(StatusCode::LengthRequired, "Content length required")
        }
    }
    pub fn create_response(&self, request: Result<&Request,&RequestError>,database: &mut DataBase) -> Response{
        match request {
            Ok(request) => self.create_ok_response(request, database),
            Err(e) => self.create_err_response(e)
        }
    }

    pub fn file_not_found() -> Response{
        let body = match fs::read_to_string("file_not_found.html"){
            Ok(file) => file,
            Err(e) => "File Not Found".to_string()
        };
    
        let status_code = StatusCode::NotFound;
        let status_msg = "File Not Found";
        let mut header: HashMap<String, String> = HashMap::new();
        header.insert("Content-Length".to_string(), body.len().to_string());
        header.insert("Content-Type".to_string(), "text/html".to_string()); 
    
        Response::new_v(status_code, status_msg, header, body)
    }

    fn create_post_request(&self) -> Response{
        todo!()
    }

    fn create_put_request(&self) -> Response{
        todo!()
    }

    fn create_delete_request(&self) -> Response{
        todo!()
    }
}