
use rust_web_server::{core::{routing::Router, server::Server}, http::Response};

fn main() {
    let mut server = Server::new();

    server.get("/".to_string(), |_req, res: &mut Response| {
        res.set_status(200);
        res.send("Hello, world! Welcome to the rust community!");
    });

    let mut router = Router::new();
    router.get("/hello".to_string(), |req, res: &mut Response| {
        println!("{:#?}", req.get_headers());
        println!("{}", req.query_params.get("page").unwrap());
        res.set_status(200);
        res.send("Hello, Isa! Welcome to the rust community!");
    });

    server.use_routes(router);

    server.run("127.0.0.1", Some(8000), None);
}
