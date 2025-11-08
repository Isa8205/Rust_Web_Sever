use std::{io::Write, net::TcpListener, sync::{Arc, Mutex}};

use crate::{core::routing::{Route, Router}, http::{Request, Response, handle_connection}, threadpool::ThreadPool};

pub struct Server {
    routes: Arc<Mutex<Vec<Route>>>,
}

impl Server {
    /// Creates a new Server
    pub fn new() -> Server {
        let default_route = Route {
            method: "GET".to_string(),
            path: "/".to_string(),
            handler: Arc::new(Mutex::new(Box::new(|_req: Request, res: &mut Response| {
                res.set_status(200);
                res.send("Hello, world! Welcome to the rust community!");
            }))),
        };
        
        Server {
            routes: Arc::new(Mutex::new(vec![default_route])),
        }
    }

    /// Adds route endpoints to the server config.
    ///
    /// # Examples
    ///
    pub fn use_routes(&mut self, routes: Router) {
        let route_arr = routes.get_routes(); 
        if let Some(routes) = route_arr {
            let new_routes = Arc::new(Mutex::new(routes));
            self.routes = new_routes;
        }
    }

    /// Creates a route with the `GET` method and adds it to the routes in `Server`
    /// 
    /// # Example
    pub fn get(&mut self, path: String, handler: impl FnMut(Request, &mut Response) + Send + 'static) {
        let route = Route {
            method: "GET".to_string(),
            path,
            handler: Arc::new(Mutex::new(Box::new(handler)))
        };

        let mut routes_guard = self.routes.lock().unwrap();
        routes_guard.push(route);
    }

    /// Creates a route with the `POST` method and adds it to the routes in `Server`
    /// 
    /// # Example
    pub fn post(&mut self, path: String, handler: impl FnMut(Request, &mut Response) + Send + 'static) {
        let route = Route {
            method: "POST".to_string(),
            path,
            handler: Arc::new(Mutex::new(Box::new(handler)))
        };

        let mut routes_guard = self.routes.lock().unwrap();
        routes_guard.push(route);
    }

    /// Starts the server listening on the given address and port
    ///
    /// # Examples
    ///
    ///
    pub fn run(&self, addr: &str, port: Option<u16>, thread_count: Option<usize>) {
        let address = match port {
            Some(port) => format!("{}:{}", addr, port),
            None => {
                println!("Port not provided. Defaulting to port 8080");
                format!("{}:8080", addr)
            }
        };
        let listener;
        match TcpListener::bind(&address) {
            Ok(l) => listener = l,
            Err(e) => {
                println!("Failed to bind to address {}", &address);
                eprintln!("Error: {}", e);
                return;
            }
        };

        println!("Listening on http://{}", &address);

        let thread_size = match thread_count {
            Some(size) => size,
            None => {
                println!("Thread count not provided. Defaulting to 4 threads");
                4
            }
        };
        let pool = ThreadPool::new(thread_size);

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(s) => Arc::new(s),
                Err(e) => {
                    println!("Failed to accept connection");
                    eprintln!("Error: {}", e);
                    continue;
                }
            };

            let thread_stream = Arc::clone(&stream);
            let routes = Arc::clone(&self.routes);
            pool.execute(move || {
                let (req, mut res) = handle_connection(thread_stream);

                // Lock the routes to find a matching route, but clone only the handler Arc so we don't
                // need mutable access to the routes after the lookup.
                let maybe_handler = {
                    let routes_guard = routes.lock().unwrap();
                    routes_guard.iter()
                        .find(|route| route.method == req.method && route.path == req.path)
                        .map(|r| r.handler.clone())
                };

                if let Some(handler_arc) = maybe_handler {
                    let mut handler = handler_arc.lock().unwrap();
                    (*handler)(req, &mut res);
                }

                // Serve a response (demo)
                let status = format!("HTTP/1.1 {} OK", res.status);
                let contents = format!("<h1>{}</h1>", res.body.unwrap_or("Could not find response body".to_string()));
                let content_len = contents.len();

                let response = format!("{status}\r\nContent-Length: {content_len}\r\n\r\n{contents}");
                stream.as_ref().write_all(response.as_bytes()).unwrap();
            })
        }
    }
}
