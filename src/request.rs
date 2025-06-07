//! Requests Structure
use std::{collections::HashMap};
pub struct Request {
    pub status_line: StatusLine,
    pub headers: HashMap<String, String>,
    pub body: String,
}
pub struct StatusLine {
    pub method: String,
    pub path: String,
    // pub params: HashMap<String, String>,
    pub http_version: String,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    TRACE,
    HEAD,
    PATCH,
    CONNECT,
}

impl StatusLine {
    fn new (status_line: String) -> StatusLine {
        let split: Vec<&str> = status_line.split(' ').collect();
        let new_status_line = StatusLine {
            method: String::from(split[0]),
            path: String::from(split[1]),
            http_version: String::from(split[2]),
        };
        new_status_line
    }
}

impl Request {
    /// Parse string into a request structure.
    pub fn from_string (mut buffer: String) -> Self {
        buffer = buffer.replace("\r", "");
        let string_buffer: Vec<&str> = buffer.split('\n').collect();

        let status_line = StatusLine::new(String::from(string_buffer[0]));

        let mut headers: HashMap<String, String> = HashMap::new();
        let headers_from_buffer = Vec::from(&string_buffer[1..string_buffer.len() - 2]);
        for header_item in headers_from_buffer {
            let header_item: Vec<&str> = header_item.split(": ").collect();
            headers.insert(header_item[0].to_string(), header_item[1].to_string());
        }
        
        let body = String::from(string_buffer[string_buffer.len() -1]);

        Request {
            status_line,
            headers,
            body,
        }
    }
}