// Thread Pool
use crate::{BoundingBoxNode, Hittable, Vec3};
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
struct Job {
    fun: Box<dyn FnBox + Send + 'static>,
    arg: Arc<BoundingBoxNode>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

trait FnBox {
    fn call_box(self: Box<Self>, a: Arc<BoundingBoxNode>) -> PixelData;
}

pub struct PixelData {
    pub pixel: Vec3,
    pub x: usize,
    pub y: usize,
}

impl<F: FnOnce(Arc<BoundingBoxNode>) -> PixelData> FnBox for F {
    fn call_box(self: Box<F>, a: Arc<BoundingBoxNode>) -> PixelData {
        (*self)(a)
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
                    let result = job.fun.call_box(job.arg);
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
    pub fn new(size: usize, pixel_sender: Arc<Mutex<mpsc::Sender<PixelData>>>) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, receiver.clone(), pixel_sender.clone()));
        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F, a: Rc<BoundingBoxNode>)
    where
        F: FnOnce(Rc<BoundingBoxNode>) -> PixelData + Send + 'static,
    {
        let job = Job {
            fun: Box::new(f),
            arg: Arc::new(BoundingBoxNode::zero()),
        };
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
