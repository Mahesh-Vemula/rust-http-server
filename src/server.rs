use std::net::{TcpListener, SocketAddr, TcpStream};
use std::io::Error;
use std::io::Read;
//use super::Request;
use crate::http::Request;
use std::convert::TryFrom;

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

        loop{
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Reqeust receive: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => println!("Failed to parse: {}", e)
                            }
                        }
                        Err(e) => println!("Reading error: {}", e)
                    }
                }
                Err(e) => println!("Failed: {}", e)
            }
        }
    }
}