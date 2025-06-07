//! HTTP Server with TCP connection and Gateway
use thiserror::Error;
use std::net::{TcpListener, TcpStream};
use std::error::Error;
use std::io::{BufReader, Read, Write};
use std::net::SocketAddr;
use std::net::{Ipv4Addr};
use crate::response::Response;
use crate::request::{Request};
use crate::router::Router;
use crate::thread_pool::ThreadPool;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Server Error: \nPoolCreationError: {0}")]
    PoolCreationError(String),
    #[error("Server Error: \nThreadCreationError: {0}")]
    ThreadCreationError(String),
    #[error("Server Error: \nReadTCPStreamError: {0}")]
    ReadTCPStreamError(String),
}

pub struct Server {
    host: Ipv4Addr,
    port: u16,
    threads: usize,
    router: Router,
}

impl Server {
    pub fn new (port: Option<u16>, host: Option<&'static str>, threads: Option<usize> ) -> Self {
        let default_port: u16 = 5050;
        let default_host: &str = "127.0.0.1";
        let default_thread_size: usize = 4;

        Server {
            host: host.unwrap_or(default_host).parse::<Ipv4Addr>().unwrap(),
            port: port.unwrap_or(default_port),
            threads: threads.unwrap_or(default_thread_size),
            router: Router::new()
        }
    }

    pub fn listen (&mut self) -> Result<(), Box<dyn Error>> {
        let addr = SocketAddr::from((self.host, self.port));
        let listener = TcpListener::bind(addr)?;
        let thread_pool = ThreadPool::new(self.threads)?;


        loop {
            let (stream, _) = listener.accept()?;
            let router_copy = self.router.clone();

            println!("Server has been running at address: {}", addr);
            thread_pool.execute(|| handle_connection(stream, router_copy));
        }
    }
}

const TCP_BUFFER_SIZE: usize = 1024;

fn handle_connection(mut stream: TcpStream, router: Router) -> Result<(), ServerError> {
    let request = read_request(&stream)?;
    let response = router.route(request);
    send_response(&mut stream, response)?;
    Ok(())
}

fn read_request(stream: &TcpStream) -> Result<Request, ServerError> {
    let mut reader = BufReader::new(stream);
    let mut buffer = [0; TCP_BUFFER_SIZE];

    let bytes_read = reader.read(&mut buffer)
        .map_err(|_| ServerError::ReadTCPStreamError(String::from("Failed to read from TCP Stream")))?;

    let raw_request = String::from_utf8((&buffer[..bytes_read]).to_vec())
        .map_err(|_| ServerError::ReadTCPStreamError(String::from("Invalid UTF-8 sequence")))?;

    Ok(Request::from_string(raw_request))
}

fn send_response(stream: &mut TcpStream, response: Response) -> Result<(), ServerError> {
    stream.write_all(response.to_http().as_bytes())
        .map_err(|_| ServerError::ReadTCPStreamError(String::from("Failed to write response")))?;
    Ok(())
}

impl Server {
    pub fn get<F>(&mut self, path: &str, handler: F)
    where F: Fn(Request) -> Response + Send + Sync + 'static
    {
        self.router.add_route("GET", path, handler);
    }

    pub fn post<F>(&mut self, path: &str, handler: F)
    where F: Fn(Request) -> Response + Send + Sync + 'static
    {
        self.router.add_route("POST", path, handler);
    }
}
