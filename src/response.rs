use std::{
    io::Error, fs, io::{prelude::*, BufRead, BufReader,Lines}, net::{TcpListener, TcpStream}, result

};

use crate::requests::{self, Method, Request};
use std::collections::HashMap;

use crate::router::TargetFunction;

pub enum StatusCode{
    OK,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    LengthRequired,
    HTTPVersionNotSupported,
}

pub struct Response{
    pub version: &'static str,
    pub status_code: StatusCode,
    pub status_msg: &'static str,
    pub header: HashMap<String, String>,
    pub body: String,    
}

impl Response {
    //Impliments version 1.1
    pub fn new_v(status_code: StatusCode, status_msg: &'static str, header: HashMap<String,String>, body: String) -> Self{
        Self { version: "HTTP/1.1", status_code, status_msg, header, body}
    }


    pub fn ok(header: HashMap<String,String>, body: String) -> Self{
        Self {version: "HTTP/1.1", status_code: StatusCode::OK, status_msg: "Ok", header, body}
    }
}

pub struct CreateResponseString{
}

impl CreateResponseString {
    pub fn new(response: Response) -> String{
        let status_code_string = match response.status_code{
            StatusCode::OK => "200",
            StatusCode::BadRequest => "400",
            StatusCode::NotFound => "404",
            StatusCode::MethodNotAllowed => "405",
            StatusCode::LengthRequired => "411",
            StatusCode::HTTPVersionNotSupported => "505"
        };
        let mut msg = format!("{} {} {}\r\n",response.version,status_code_string,response.status_msg);

        for (key,value) in response.header{
            msg.push_str(format!("{}: {}\r\n",key,value).as_str());
        }
        msg.push_str("\r\n");
        msg.push_str(&response.body);

        return msg;
    }
}