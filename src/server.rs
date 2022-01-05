use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&self, req: &Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

#[derive(Debug)]
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(&self, handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("Listening on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Accepted connection at addr: {}", addr);
                    let mut buf = [0; 8192];

                    match stream.read(&mut buf) {
                        Ok(size) => {
                            println!("Bytes read: {}", size);
                            println!("Request: {}", String::from_utf8_lossy(&buf));

                            let response = match Request::try_from(&buf[..]) {
                                Ok(req) => handler.handle_request(&req),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e)
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    };
                }
                Err(e) => println!("Failed to accept connection: {}", e),
            }
        }
    }
}
