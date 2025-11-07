
use rust_web_server::core::{response::Response, server::Server};

fn main() {
    let mut server = Server::new();

    server.get("/".to_string(), |_req, res: &mut Response| {
        res.set_status(200);
        res.send("Hello, world! Welcome to the rust community!");
    });

    server.run("127.0.0.1", Some(8000), None);
}
