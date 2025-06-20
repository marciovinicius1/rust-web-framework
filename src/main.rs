mod utils;
mod request;
mod server;
mod router;
mod response;
mod error;
mod thread_pool;
mod radix_tree;

use crate::response::Response;
use crate::server::Server;

fn main() {
    println!("Logs from server will appear here!");
    let mut server = Server::new(Some(5050), Some("0.0.0.0"), Some(4));

    server.get("/", |_req| {
        Response::text("Hello from Rust Web Framework!")
    });

    server.get("/about", |_req| {
        Response::text("This is the about page.")
    });
    
    if let Err(e) = server.listen() {
        panic!("{}",e)
    }
}