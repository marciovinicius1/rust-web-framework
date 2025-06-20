//! Routes Store and Management
use crate::request::{Request, Method as RequestMethod};
use crate::response::Response;
use crate::radix_tree::RadixTree;
use std::sync::Arc;

pub type Handler = Arc<dyn Fn(Request) -> Response + Send + Sync>;
pub struct Router {
    routes: RadixTree,
}

fn string_to_method(s: &str) -> Result<RequestMethod, String> {
    match s.to_uppercase().as_str() {
        "GET" => Ok(RequestMethod::GET),
        "POST" => Ok(RequestMethod::POST),
        "PUT" => Ok(RequestMethod::PUT),
        "DELETE" => Ok(RequestMethod::DELETE),
        "OPTIONS" => Ok(RequestMethod::OPTIONS),
        "TRACE" => Ok(RequestMethod::TRACE),
        "HEAD" => Ok(RequestMethod::HEAD),
        "PATCH" => Ok(RequestMethod::PATCH),
        "CONNECT" => Ok(RequestMethod::CONNECT),
        _ => Err(format!("Unsupported HTTP method: {}", s)),
    }
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: RadixTree::new(),
        }
    }
    pub fn add_route<F> (&mut self, method: &str, path: &str, handler: F)
    where F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        match string_to_method(method) {
            Ok(method) => {
                self.routes.root.insert(path, method, handler);
            }
            Err(e) => {
                panic!("Failed to add Route: {}", e);
            }
        }
    }
    pub fn route (&self, mut req: Request) -> Response {
        match string_to_method(&req.status_line.method) {
            Ok(method) => {
                match self.routes.root.match_route(&req.status_line.path, method)
                {
                    Ok(route_response) => {
                        req.params = route_response.params;
                        (route_response.handler)(req)
                    }
                    Err(_) => {
                        Response::text(&"404 Not Found".to_string()).with_status(404)
                    }
                }
            }
            Err(_) => {
                Response::text(&"400 Bad Request - Invalid Method".to_string()).with_status(400)
            }
        }
    }
}