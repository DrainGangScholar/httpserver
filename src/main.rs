use request_handler::RequestHandler;
use server::Server;
use std::env;
mod http;
mod request_handler;
mod server;
fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(RequestHandler::new(public_path));
}
