use std::{
    sync::{Arc, Mutex, mpsc}, 
    thread::{self, JoinHandle}
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} recieved a job. Executing!");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected. Shutting down!");
                        break;
                    }
                    
                }
            }
        });

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let reciver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciver)));
        }

        ThreadPool { 
            workers, 
            sender: Some(sender) 
        }
    }

    /// Execute a function on a worker thread in the pool
    /// 
    /// # Panics
    /// 
    /// This function will panic if the pool has been shut down
    /// 
    /// # Examples
    /// 
    /// 
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static  
    {
        let job = Box::new(f);

        if let Some(sender) = &self.sender {
            if let Err(e) = sender.send(job) {
                eprintln!("Failed to execute job: {e}");
            }
        } else {
            eprintln!("Job not executed. ThreadPool is shut down.");
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}