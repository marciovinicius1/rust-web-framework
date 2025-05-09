//! HTTP Server with TCP connection and Gateway
use thiserror::Error;
use std::net::{TcpListener, TcpStream};
use std::error::Error;
use std::io::{BufReader, Read, Write};
use std::net::SocketAddr;
use std::net::{Ipv4Addr};

use crate::response::Response;
use crate::handle_connection;
use crate::request::Request;
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
            println!("Server has been running at address: {}", addr);
            thread_pool.execute(|| handle_connection(stream));
        }
    }

    fn handle_connection(mut stream: TcpStream, router: Router) -> Result<(), ServerError> {
        let mut reader = BufReader::new(&stream);
        let mut buffer = [0; 1024];
        let content_len = reader.read(&mut buffer);

        let string_buffer = match content_len {
            Ok(len) => String::from_utf8((&buffer[..len]).to_vec())
                .map_err(|_| ServerError::ReadTCPStreamError(String::from("Cannot read data from TCP Stream.")))?,
            Err(_) => {
                return Err(ServerError::ReadTCPStreamError(String::from("Cannot read data from TCP Stream.")));
            }
        };

        let request = Request::raw_req(string_buffer);
        let response = router.route(request);

        stream.write_all(response.to_http().as_bytes()).unwrap();
        Ok(())
    }
}


impl Server {
    pub fn get<F>(&mut self, path: &str, handler: F)
    where F: Fn(Request) -> Response + Send + Sync + 'static
    {
        self.router.add_route("GET", path, handler);
    }
}