use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::Write;
use std::collections::HashMap;

use crate::get_requests;
use crate::database::{DataBase};



use crate::response::{Response, CreateResponseString};
use crate::requests::Request;

use crate::router::TargetFunction;
use crate::router::Router;
pub struct Server{
    listener: TcpListener,
    router:Router,
    database: DataBase,
}

impl Server {
    pub fn new(addr: &str) -> io::Result<Self>{
        let listener = TcpListener::bind(addr)?;
        Ok(Self{listener, router: Router::new(),database: DataBase::new()})
    }
    pub fn listen(&mut self){
        for stream in self.listener.incoming(){
            if let Ok(mut request_stream) = stream{
                let request = Request::new(&request_stream);
                let response_data = self.router.create_response(request.as_ref(),&mut self.database);
                let response_string = CreateResponseString::new(response_data);
                let res = request_stream.write_all(response_string.as_bytes());
                request_stream.flush().unwrap();
            }
        }
    }
}