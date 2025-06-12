use std::{
    fmt,
   fs, io::{prelude::*, BufRead, BufReader,Lines}, net::{TcpListener, TcpStream}, result

};

use std::error::Error;

use std::collections::HashMap;
#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE
}

#[derive(Debug)]
pub enum RequestError{
    FormatingError(&'static str),
    MethodNotAllowed,
    LengthRequired,
}

impl std::fmt::Display for RequestError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            RequestError::FormatingError(msg) => write!(f,"{}",msg),
            RequestError::MethodNotAllowed => write!(f,"{}","Method not allowed"),
            RequestError::LengthRequired => write!(f,"{}","Content length required")
        }
    }
}

impl From<std::io::Error> for RequestError {
    fn from(e: std::io::Error) -> Self {
        RequestError::FormatingError("Unexpected EOF")
    }
}


impl Error for RequestError{}

struct RequestParser<'a>{
    request_buf_reader: BufReader<&'a TcpStream>
}

impl<'a> RequestParser<'a>{
    fn new(stream: &'a TcpStream) -> Self{
        Self{request_buf_reader: BufReader::new(stream)}
    }
    fn parse_start(&mut self) -> Result<(String, String, String),RequestError> {

        let request_start_line = self.request_buf_reader.by_ref().lines().next().ok_or(RequestError::FormatingError("No data provided"))??;

        let mut start_line_iter = request_start_line.split_whitespace();

        let request_method = start_line_iter.next().ok_or(RequestError::FormatingError("No method provided"))?;

        let request_target = start_line_iter.next().ok_or(RequestError::FormatingError("No target provided"))?;

        let request_version =  start_line_iter.next().ok_or(RequestError::FormatingError("No version provided"))?;

        if start_line_iter.next().is_some(){
            return Err(RequestError::FormatingError("Too much data provided on first line"))
        }       
        return Ok((request_method.to_string(),request_target.to_string(),request_version.to_string()));
    }
    fn parse_header(&mut self) -> Result<HashMap<String, String>,RequestError>{
        let mut header_hash_map: HashMap<String, String> = HashMap::new();
        for line in self.request_buf_reader.by_ref().lines(){
            let line = match line{
                Ok(x) => x.to_lowercase(),
                Err(e) => return Ok(header_hash_map)
            };
            if line.trim().is_empty(){
                break;  //end of header
            }
            let mut line_itr = line.split(": ");
            let key = line_itr.next().ok_or(RequestError::FormatingError("Header not formatted correctly"))?;
            let value = line_itr.next().ok_or(RequestError::FormatingError("Header not formatted correctly"))?;
            header_hash_map.insert(key.to_string(), value.to_string());
        }
        return Ok(header_hash_map);
    }
    fn parse_body(&mut self, content_length: usize) -> String{
        let mut body_buf  = vec![0; content_length];
        self.request_buf_reader.read_exact(&mut body_buf);
        return String::from_utf8_lossy(&body_buf).to_string()
    }
}
pub struct Request{
    pub method: Method,
    pub target: String,
    pub version: String,
    pub header: HashMap<String, String>,
    pub body: String,    
}

impl<'a> Request{
    pub fn new(stream: &'a TcpStream) -> Result<Self,RequestError>{     
        let mut request_parser = RequestParser::new(stream);
        let (request_method_as_string,request_target,request_version) = request_parser.parse_start()?;
        
        let request_method;
        match request_method_as_string.as_str(){
            "GET" => request_method = Method::GET,
            "POST" => request_method = Method::POST,
            "PUT" => request_method = Method::PUT,
            "DELETE" => request_method = Method::DELETE,
            _ => return Err(RequestError::MethodNotAllowed)
        }
        let header = request_parser.parse_header()?;
        let mut body = String::new();
        if !matches!(request_method, Method::GET){
            let body_length = header.get("content-length").ok_or(RequestError::LengthRequired)?;
            let body_length = body_length.parse::<usize>().map_err(|e| RequestError::LengthRequired)?;
            body = request_parser.parse_body(body_length);
        }

        return Ok(Self{
            method: request_method,
            target: request_target,
            version: request_version,
            header: header,
            body
        });
    }
}