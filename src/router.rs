//! Routes Store and Management
use std::collections::HashMap;
use std::sync::Arc;

use crate::request::Request;
use crate::response::Response;

type Handler = Arc<dyn Fn(Request) -> Response + Send + Sync>;
#[derive(Clone)]
pub struct Router {
    routes: HashMap<String, Handler>,
}
impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }
    pub fn add_route<F> (&mut self, method: &str, path: &str, handler: F) where
    F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        let key = format!("{} {}", method, path);
        self.routes.insert(key, Arc::new(handler));
    }
    pub fn route (&self, req: Request) -> Response {
        let key = format!("{} {}", req.status_line.method, req.status_line.path);
        if let Some(handler) = self.routes.get(&key) {
            handler(req)
        } else {
            Response::text("404 Not Found").with_status(404)
        }
    }
}