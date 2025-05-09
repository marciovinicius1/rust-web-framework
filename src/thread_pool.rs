use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::server::ServerError;
use crate::server::ServerError::ThreadCreationError;

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
            Err(_) => Err(ThreadCreationError(format!("Cannot create the worker id: {}", id))),
        }
    }
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
            return Err(ServerError::PoolCreationError(String::from("The size of thread pool must be grater than 0.")))
        };

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        //TODO: pass the id of worker with default message in error case worker::new() | remove unwrap().
        for i in 0..size {
            let worker = Worker::new(i, Arc::clone(&receiver));
            match worker {
                Ok(worker) => workers.push(worker),
                Err(e)=> return Err(e)
            }
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