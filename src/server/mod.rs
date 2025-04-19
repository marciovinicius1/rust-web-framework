
// DONE FINALIZAR MULTITHREADING
// TODO MELHORAR LANÇAMENTO DE ERROS
// TODO ASSINCRONISMO
// TODO IMPLEMENTAÇÃO DE TESTES

mod errors;
use std::net::TcpListener;
use std::{io, thread, thread::{JoinHandle}, sync::{mpsc, Arc, Mutex}};
use crate::handle_connection;
use std::net::SocketAddr;
use std::net::{Ipv4Addr};
use crate::server::ServerError::ThreadCreationError;

    pub enum ServerError {
        PoolCreationError,
        ThreadCreationError,
    }
    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Job>,
    }
    impl ThreadPool {
        /// Create a new ThreadPool.
        ///
        /// The size is the number of threads in the pool.
        ///
        /// # Panics
        ///
        /// The `new` function will panic if the size is zero.
        ///
        pub fn new(size: usize) -> Result<ThreadPool, ServerError> {
            if size == 0 {
                return Err(ServerError::PoolCreationError)
            };

            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers: Vec<Worker> = Vec::with_capacity(size);
            //TODO: pass the id of worker with default message in error case worker::new() | remove unwrap().
            for i in 0..size {
                workers.push(Worker::new(i, Arc::clone(&receiver))?)
            }
            Ok( ThreadPool { workers, sender })
        }

        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    struct Worker {
        id: usize,
        thread: JoinHandle<()>
    }

    impl Worker {
        fn new (id:usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, ServerError> {
            let builder = thread::Builder::new();
            let thread = builder.spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job; executing.");

                job();
            });

            match thread {
                Ok(thread) => Ok(Worker { id, thread }),
                Err(T) => Err(ThreadCreationError),
            }
        }
    }

    pub struct Server {
        pub host: Ipv4Addr,
        pub port: u16,
        pub threads: usize
    }

    impl Server {
        pub fn new (port: Option<u16>, host: Option<&'static str>, threads: Option<usize> ) -> Server {
            let default_port:u16 = 5050;
            let default_host: &str = "127.0.0.1";
            let default_thread_size: usize = 4;

            let port = port.unwrap_or(default_port);
            let host = host.unwrap_or(default_host).parse::<Ipv4Addr>().unwrap();
            let threads = threads.unwrap_or(default_thread_size);

            Server { host, port, threads }
        }

        pub fn init (&mut self) -> Result<(), io::Error> {

            let addr = SocketAddr::from((self.host, self.port));
            let listener = TcpListener::bind(addr)?;
            let thread_pool = ThreadPool::new(self.threads);

            match thread_pool {
                Ok(pool) => {
                    loop {
                        let (stream, _) = listener.accept()?;
                        println!("Server has been running at address: {}", addr);
                        pool.execute(|| handle_connection(stream));
                    }
                }
                Err(ServerError::PoolCreationError) => panic!("Error: ThreadPool size must be greater than 0."),
                Err(ThreadCreationError) => panic!("Error: Fail to build thread in worker."),
                _ => panic!("Unknown Error")
            }
        }
    }
