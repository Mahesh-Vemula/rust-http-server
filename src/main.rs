#![allow(dead_code)]

mod server;
mod http;

use server::Server;
use http::Request;

fn main() {
    println!("Hello, world!");
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}


