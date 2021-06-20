#![allow(dead_code)]

mod server;
mod http;
mod website_handler;

use server::Server;
use http::Request;
use crate::website_handler::WebsiteHandler;

fn main() {
    println!("Hello, world!");
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}


