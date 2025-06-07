mod utils;
mod request;
mod server;
mod router;
mod response;
mod error;
mod thread_pool;
mod radix_tree;

#[allow(unused_imports)]
use std::{collections::HashMap, io::{ Read, Write, BufReader, BufRead}, net::TcpStream};
use std::{ fs, env };
use utils::compression::compress_string;
use crate::response::Response;
use crate::server::Server;

#[derive(Debug)]
#[allow(dead_code)]
struct StatusLine {
    method: String,
    route: String,
    http_version: String,
}

impl StatusLine {
    fn new (status_line: String) -> StatusLine {
        let split: Vec<&str> = status_line.split(' ').collect();
        let new_status_line = StatusLine {
            method: String::from(split[0]),
            route: String::from(split[1]),
            http_version: String::from(split[2]),
        };
        new_status_line
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct HTTPRequest {
    status_line: StatusLine,
    headers: HashMap<String,String>,
    body: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct HTTPResponse {
    status_line: String,
    headers: String,
    body: String,
}

fn handle_connection (mut stream: TcpStream) {
    println!("accepted new connection");
    let _response_200 = "HTTP/1.1 200 OK\r\n\r\n";
    let _response_404 = "HTTP/1.1 404 Not Found\r\n\r\n";
    let _response_user_agent= "HTTP/1.1 200 OK \r\n Content-Type: text/plain\r\n Content-Length: 12\r\n\r\n";

    //received bytes
    let mut reader = BufReader::new(&stream);
    let mut buffer = vec![0;1024];
    let len = reader.read(&mut buffer)
        .expect("cannot read from stream");
    let mut string_buffer = String::from_utf8((&buffer[..len]).to_vec())
        .expect("cannot transform buffer into string");
    string_buffer = string_buffer.replace("\r", "");
    let string_buffer: Vec<&str> = string_buffer.split('\n').collect();

    //set status line
    let status_line = StatusLine::new(String::from(string_buffer[0]));

    //set headers
    let mut headers: HashMap<String, String> = HashMap::new();
    let headers_from_buff = Vec::from(&string_buffer[1..string_buffer.len() - 2]);
    for header_item in headers_from_buff {
        let header_item: Vec<&str> = header_item.split(": ").collect();
        headers.insert(header_item[0].to_string(), header_item[1].to_string());
    }

    //set body
    let body = String::from(string_buffer[string_buffer.len() -1]);

    // create request instance
    let req = HTTPRequest {
        status_line,
        headers,
        body,
    };
    println!("{:?}", req);

    let route = req.status_line.route.clone();
    let routes: Vec<&str> = route.split('/').collect();
    let url_data = routes[routes.len() - 1];
    let mut accept_encodings: Vec<&str> = Vec::new();
    for (key, value) in &req.headers {
        match key.as_str() {
            "Accept-Encoding" => {
                accept_encodings = value.split(", ").collect();
            }
            _ => ()
        }
    }
    println!("accept_encodings {:?}", accept_encodings);

    match req {
        HTTPRequest { status_line, headers:_, body:_ } if status_line.route == "/" =>
            stream.write_all(_response_200.as_bytes()).expect("cannot send response to client in source route"),
        HTTPRequest { status_line, headers: _, body:_ } if status_line.route == format!("/echo/{}", url_data) => {
            let mut res = String::new();
            res = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", url_data.len(), url_data);

            if accept_encodings.contains(&"gzip") {
                let compress_body= compress_string(url_data).expect("error when compress body");
                let res = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Encoding: gzip\r\nContent-Length: {}\r\n\r\n",compress_body.len());
                stream.write(res.as_bytes()).expect("cannot send response to client with gzip compress");
                stream.write(&compress_body).unwrap();
                return;
            }
            stream.write_all(res.as_bytes()).expect("cannot send response to client in echo route")
        },
        HTTPRequest { status_line, headers, body:_ } if status_line.route == "/user-agent" => {
            if let Some((_k, v )) = headers.get_key_value("User-Agent") {
                let r: String = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", v.len(), v);
                stream.write_all(r.as_bytes()).expect("cannot send response to client in user-agent route")
            }
        },
        HTTPRequest { status_line, headers:_, body:_ } if status_line.method == "GET" && status_line.route == format!("/files/{}", url_data) => {

            let env_args: Vec<String> = env::args().collect();
            let mut dir: String;

            if env_args.len() > 1 {
                dir = env_args[2].clone();
            } else {
                dir = String::from("./tmp/");
            }

            dir.push_str(&url_data);
            let file = fs::read(dir);


            match file {
                Ok(fc) => {
                    let r = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n", fc.len(), String::from_utf8(fc).expect("file content"));
                    stream.write_all(r.as_bytes()).expect("cannot send response to client in echo route")
                }
                Err(..) => stream.write_all(_response_404.as_bytes()).unwrap()
            }
        },
        HTTPRequest { status_line, headers:_, body:_ } if status_line.method == "GET" && status_line.route == format!("/files/{}", url_data) => {

            let env_args: Vec<String> = env::args().collect();
            let mut dir: String;

            if env_args.len() > 1 {
                dir = env_args[2].clone();
            } else {
                dir = String::from("./tmp/");
            }

            dir.push_str(&url_data);

            let r = "HTTP/1.1 201 OK\r\nContent-Type: application/octet-stream\r\n";
            stream.write_all(r.as_bytes()).expect("cannot send response to client in echo route")

        },
        HTTPRequest { status_line, headers: _, body, } if status_line.method == "POST" && status_line.route == format!("/files/{}", url_data) => {

            let env_args: Vec<String> = env::args().collect();
            let mut dir: String;
            if env_args.len() > 1 {
                dir = env_args[2].clone();
            } else {
                dir = String::from("./tmp/");
            }
            dir.push_str(&url_data);
            let file = fs::write(dir, body);
            match file {
                Ok(..) => stream
                    .write_all("HTTP/1.1 201 Created\r\n\r\n".as_bytes())
                    .expect("cannot send response to client in echo route"),
                Err(..) => stream.write_all(_response_404.as_bytes()).unwrap(),
            }
        },
        _ => stream.write_all(_response_404.as_bytes()).unwrap(),
    }
}

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