use std::net::{TcpListener};
use std::io::{Read};
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;

pub trait Handler{
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e:&ParseError) -> Response{
        print!("Failed to parse request. {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server{
    addr: String,
}

impl Server{
    pub fn new(addr: String) -> Self{
        Self{
            addr,
        }
    }

    pub fn run(self, mut handler: impl Handler){
        println!("Server is running and listening on {}", self.addr);
        let listener = TcpListener::bind(self.addr).unwrap();

        loop{
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Reqeust receive: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]){
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream){
                                print!("Failed to send response: {}", e);
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