use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::server::ServerError;
use crate::server::ServerError::ThreadCreationError;

type Job = Box<dyn FnOnce() -> Result<(), ServerError> + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>
}

impl Worker {
    fn new (id:usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, ServerError> {
        let builder = thread::Builder::new();

        let thread = builder.spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    if let Err(e) = job() {
                        eprintln!("Error while executing job in worker {id}: {:?}", e);
                    }
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        match thread {
            Ok(thread) => Ok(Worker { id, thread: Some(thread) }),
            Err(_) => Err(ThreadCreationError(format!("Cannot create the worker id: {}", id))),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
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
        Ok( ThreadPool { workers, sender: Some(sender) })
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() -> Result<(), ServerError> + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            if let Err(e) = sender.send(job) {
                eprintln!("Failed to send to the thread pool: {:?}", e);
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}