use crate::requests::{self, HTTPData};

use std::{
fs, io::{prelude::*, BufRead, BufReader,Lines}, net::{TcpListener, TcpStream}, result

};

static mut DATA: Vec<String> = Vec::new();


pub struct Response<'a>{
    http_data: requests::HTTPData<'a>,
    status_code: u32,
    status_msg: String,
    content_length: usize,
    content_type: String,
 }

 impl<'a> Response<'a> {
    pub fn new(http_data: requests::HTTPData<'a>) -> Self{
        Self {http_data, status_code: 200, status_msg: "Ok".to_string(), content_length: 0, content_type: String::new()}
    }

    fn create_sucess_request(&self, body: &str) -> String{
        let status = "HTTP/1.1 200 OK";
        return format!("{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",status, self.content_length,self.content_type,body);
    }

    fn create_fail_response(&self) -> String{
        return "HTTP/1.1 404 Page not found".to_string();
    }

    pub fn create_request(&mut self) -> String{
        let response: Result<String, std::io::Error>;
        match self.http_data.method{
            requests::Method::GET => response = self.handle_get_request(),
            requests::Method::POST => {self.handle_post_request(); response = Ok(String::new())},
            _ => {response = Ok(String::new())}
        } 
        match response{
            Ok(body) => self.create_sucess_request(&body),
            Err(E) => self.create_fail_response()
        }
    }
    fn check_if_file_request(&self) -> bool{
        self.http_data.target.split(".").nth(1).is_some()
    }
    fn get_file(&mut self) -> Result<String, std::io::Error>{
        let file_extension = self.http_data.target.split(".").nth(1).unwrap();
        let file_name = &self.http_data.target;
        if let Ok(body) = fs::read_to_string(file_name){
            self.content_type = format!("text/{}",file_extension);
            self.content_length = body.len();
            return Ok(body);
        } else{
            todo!()
        }
    }
    fn handle_get_request(&mut self)-> Result<String,std::io::Error>{
        let mut body: String = String::from("");
        //only case where request does not match file name
        if self.http_data.target == ""{
            self.http_data.target = "index.html".to_string();
        }

        if self.check_if_file_request(){
            return self.get_file();
        }
        else if self.http_data.target == "data"{
            body = String::from("{\"data\": [");
            unsafe{
                for (idx, val) in DATA.iter().enumerate(){
                    body.push_str(val);
                    if idx != DATA.len() -1 {
                        body.push(',');
                    }
            }
            body.push_str("]}");
            self.content_type = "application/json".to_string();
            self.content_length = body.len();
            return Ok(body);}
        }
        return Ok(String::new());
    
    }
    fn handle_post_request(&self){
        match self.http_data.target.as_str(){
            "msg"=> self.parse_msg(),
            _ => {}
        }
    }
    fn parse_msg(&self){
        let string = self.http_data.body.split(":").nth(1).unwrap().split("}").nth(0).unwrap();
        unsafe{
            let mut arr = DATA.clone();
            arr.push(String::from(string));
            DATA = arr;
        }
    }
    
}