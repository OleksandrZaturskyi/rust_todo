use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("Listening on {}", self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Accepted connection at addr: {}", addr);

                    let mut buf = [0; 1024];

                    match stream.read(&mut buf) {
                        Ok(size) => {
                            println!("Bytes read: {}", size);
                            println!("Request: {}", String::from_utf8_lossy(&buf));

                            match Request::try_from(&buf[..]) {
                                Ok(req) => {}
                                Err(e) => println!("Failed to parse request: {}", e),
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
