use server::Server;
use http::Method;
use http::RawRequest;

mod server;
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}