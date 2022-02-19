use server::Server;
use http::request::Request;
use http::method::Method;

mod http;
mod server;


fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
// Listening for TCP connections - Udemy is where I left of tues 2.15