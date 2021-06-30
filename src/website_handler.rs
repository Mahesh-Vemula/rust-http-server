use crate::server::Handler;
use crate::http::{Response, Request, StatusCode, Method};
use std::fs;

pub struct  WebsiteHandler{
    public_path: String
}

impl WebsiteHandler{
    pub fn new(public_path:String) -> Self {
        WebsiteHandler{
            public_path
        }
    }

    fn read_file(&self, file: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file);
        println!("path received: {}", path);
        match fs::canonicalize(path){
            Ok(path) => {
                //println!("path canonicalize: {}", path.as_os_str());
                if path.starts_with(&self.public_path){
                    fs::read_to_string(path).ok()
                }else{
                    println!("Directory Traversal Attack Attempted: {}", file);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request) -> Response {

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path){
                    None => Response::new(StatusCode::NotFound, None),
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents))
                }
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}