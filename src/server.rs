
    use std::net::TcpListener;
    use std::{io, thread};
    use crate::handle_connection;
    use std::net::SocketAddr;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    pub struct Server {
        pub host: Option<&'static str>,
        pub port: u16,
    }

    impl Server {
        pub fn init (&mut self) -> (Result<(), io::Error>) {
            let port = self.port;
            let host = self.host.unwrap_or("127.0.0.1").parse::<Ipv4Addr>().unwrap();
            let addr = SocketAddr::from((host, port));
            let listener = TcpListener::bind(addr)?;

            loop {
                let (stream, _) = listener.accept()?;
                println!("Server has been running at address: {}", addr);
                thread::spawn(|| handle_connection(stream));
            }
        }
    }
