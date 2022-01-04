use server::Server;

mod http;
mod server;

fn main() {
    let addr = "127.0.0.1:8080".to_string();
    let server = Server::new(addr);
    server.run();
}
