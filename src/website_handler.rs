use crate::server::Handler;
use crate::http::{Response, Request, StatusCode, Method};

pub struct  WebsiteHandler;

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request) -> Response {

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("Home path".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("Hello".to_string())),
                _ => Response::new(StatusCode::NotFound, None)
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}