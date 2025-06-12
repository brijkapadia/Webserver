use std::{
    fmt::format, fs::{self, File}, io::{prelude::*, BufRead, BufReader,Lines}, net::{TcpListener, TcpStream}, result

};
use requests::Method;
mod requests;
mod response;
mod server;
mod get_requests;
mod router;
mod database;
mod post_requests;


const ADRR: &str = "0.0.0.0:80";



fn main() {
    let mut listener= server::Server::new(ADRR).unwrap();
    listener.listen();
}
