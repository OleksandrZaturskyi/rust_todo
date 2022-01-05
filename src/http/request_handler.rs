use super::{Method, Request, Response, StatusCode};
use crate::server::Handler;
use std::fs::{canonicalize, read_to_string};

#[derive(Debug)]
pub struct RequestHandler {
    public_path: String,
}

impl RequestHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", &self.public_path, file_path);
        match canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for RequestHandler {
    fn handle_request(&self, req: &Request) -> Response {
        match req.protocol() {
            "HTTP/1.1" => match req.method() {
                &Method::GET => match req.path() {
                    "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                    "/favicon.ico" => Response::new(StatusCode::Ok, self.read_file("favicon.ico")),
                    "/health" => Response::new(StatusCode::Ok, Some("OK".to_string())),
                    path => match self.read_file(path) {
                        Some(f) => Response::new(StatusCode::Ok, Some(f)),
                        None => Response::new(
                            StatusCode::NotFound,
                            Some(format!("{} not found", req.path())),
                        ),
                    },
                },
                _ => Response::new(StatusCode::MethodNotAllowed, None),
            },
            _ => Response::new(StatusCode::HTTPVersionNotSupported, None),
        }
    }
}
