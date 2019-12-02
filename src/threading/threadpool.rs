// Thread Pool
use crate::Vec3;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

trait FnBox {
    fn call_box(self: Box<Self>) -> PixelData;
}

pub struct PixelData {
    pixel: Vec3,
    x: usize,
    y: usize,
}

impl<F: FnOnce() -> PixelData> FnBox for F {
    fn call_box(self: Box<F>) -> PixelData {
        (*self)()
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        sender: Arc<Mutex<mpsc::Sender<PixelData>>>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    let result = job.call_box();
                    sender.lock().unwrap().send(result).unwrap();
                }
                Message::Terminate => {
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let (pixel_sender, _pixel_receiver) = mpsc::channel();
        let pixel_sender_arc = Arc::new(Mutex::new(pixel_sender));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, receiver.clone(), pixel_sender_arc.clone()));
        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> PixelData + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
