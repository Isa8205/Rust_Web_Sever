use std::sync::{Arc, Mutex};

use crate::http::{Request, Response};

pub struct Router {
    routes: Vec<Route>
}

impl Router {
    /// Instanciates a new `Router` struct. It comes with an empty vector by default.
    /// 
    /// # Example
    /// 
    /// ```
    /// use rust_web_server::core::routing::Router;
    /// 
    /// let router = Router::new();
    /// 
    /// ```
    pub fn new() -> Router {
        Router {
            routes: Vec::new()
        }
    }
    /// Creates a route with the `GET` method and adds it to the routes in `Router`
    /// 
    /// # Example
    pub fn get(&mut self, path: String, handler: impl FnMut(Request, &mut Response) + Send + 'static) {
        let route = Route {
            method: "GET".to_string(),
            path,
            handler: Arc::new(Mutex::new(Box::new(handler)))
        };

        self.routes.push(route);
    }

    /// Creates a route with the `POST` method and adds it to the routes in `Router`
    /// 
    /// # Example
    pub fn post(&mut self, path: String, handler: impl FnMut(Request, &mut Response) + Send + 'static) {
        let route = Route {
            method: "POST".to_string(),
            path,
            handler: Arc::new(Mutex::new(Box::new(handler)))
        };

        self.routes.push(route);
    }

    /// Converts the routes in `Router` to a vector of `Route`. 
    /// Should only be called by the `Server` as it consumes the routes
    pub fn get_routes(self) -> Option<Vec<Route>> {
        Some(self.routes)
    }
}

pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: Arc<Mutex<Box<dyn FnMut(Request, &mut Response) + Send + 'static>>>,
}