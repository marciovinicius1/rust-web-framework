use std::collections::HashMap;
use std::sync::Arc;
use crate::request::{Method, Request};
use crate::router::Handler;
use crate::response::Response;
pub struct RadixTree {
    pub root: Node
}
impl RadixTree {
    pub fn new() -> Self {
        RadixTree {
            root: Node::new()
        }
    }
}
pub struct Node {
    prefix: String,
    childrens: Vec<Node>,
    handlers: HashMap<Method, Handler>,
    param_key: Option<String>,
}
pub struct RouteResponse<'a> {
    pub params: HashMap<String, String>,
    pub handler: &'a Handler
}
impl Node {
    fn new() -> Self {
        Node {
            prefix: String::from("/"),
            handlers: HashMap::new(),
            param_key: None,
            childrens: Vec::new(),
        }
    }

    pub fn insert<F>(&mut self, path: &str, method: Method, handler: F) where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        let segments = path.split('/').filter(|s| !s.is_empty()).enumerate();
        let mut current_node = self;

        for (_, segment) in segments {
            if segment.chars().next() == Some(':') {
                let param_name = segment.trim_start_matches(':').to_string();
                current_node.param_key = Some(param_name);
                continue
            }

            let child_position = current_node
                .childrens
                .iter()
                .position(|child| child.prefix == segment);

            if let Some(pos) = child_position {
                current_node = &mut current_node.childrens[pos];
                continue
            }

            let new_node = Node {
                prefix: segment.to_string(),
                childrens: Vec::new(),
                handlers: HashMap::new(),
                param_key: None,
            };

            current_node.childrens.push(new_node);
            current_node = current_node
                .childrens
                .last_mut()
                .expect("Acabamos de adicionar este nó");
        }

        current_node.handlers.insert(method, Arc::new(handler));
    }

    pub fn match_route<F>(&mut self, path: &str, method: Method) -> RouteResponse where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        let segments = path.split('/').filter(|s| !s.is_empty()).enumerate();
        let mut current_node = self;
        let mut params: HashMap<String, String>  = HashMap::new();

        for (_, segment) in segments {
            let child_position = current_node
                .childrens
                .iter()
                .position(|child| child.prefix == segment);

            match (child_position, &current_node.param_key) {
                (Some(pos), None) => {
                    current_node = &mut current_node.childrens[pos];
                    continue
                }
                (None, Some(param_key)) => {
                    params.insert(param_key.clone(), segment.to_string());
                }
                _ => {
                    todo!("no match route");
                }
            }
        }

        if let Some(handler) = current_node.handlers.get(&method) {
            RouteResponse {
                handler,
                params,
            }
        } else {
            todo!("no match handler")
        }
    }
}


