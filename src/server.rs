use std::net::{TcpListener, SocketAddr, TcpStream};
use std::io::Error;

pub struct Server{
    addr: String,
}

impl Server{
    pub fn new(addr: String) -> Self{
        Self{
            addr,
        }
    }

    pub fn run(self){
        println!("Server is running and listening on {}", self.addr);
        let listener = TcpListener::bind(self.addr).unwrap();
    }
}