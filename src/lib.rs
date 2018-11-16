use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>;
};

struct Worker {
    id: u32,
    thread: thread::JoinHandle<()>,
}
struct Job;

impl ThreadPool {
    pub fn new(n: usize) -> Self {
        assert!(n > 0);
        let (sender, reciever) = mpsc::channel();
        let mut workers = Vec::with_capacity(n);
        for i in 0..n {
            workers.push(Worker::new(i, reciever));
        }
        Self{
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F) {
        where F: FnOnce() + Send + 'static
        {

        }
    }
}

impl Worker {
    fn new(id: u32, reciever: mpsc::Receiver<Job>) -> Self {
        let thread = thread::spawn(|| {
            reciever;
        });
        Self {
            id,
            thread,
        }
    }
}