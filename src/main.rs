use http::RequestHandler;
use server::Server;
use std::env::var;

mod http;
mod server;

fn main() {
    let addr = var("ADDRESS").unwrap_or("127.0.0.1:8080".to_string());
    let public_path =
        var("PUBLIC_PATH").unwrap_or(format!("{}/public", env!("CARGO_MANIFEST_DIR")));

    let server = Server::new(addr);
    let handler = RequestHandler::new(public_path);

    server.run(handler);
}
