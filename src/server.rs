
    use std::net::TcpListener;
    use std::io;
    pub struct Server {
        pub host: Option<&'static str>,
        pub port: u16,
    }

    impl Server {
        pub fn init (&mut self) -> Result<TcpListener, io::Error> {
            let port = self.port;
            let host = self.host.unwrap_or("127.0.0.1");

            let addr = (host, port);
            let listener = TcpListener::bind(addr)?;
            Ok(listener)
        }
    }
