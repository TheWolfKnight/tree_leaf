
use std::thread::{self, JoinHandle};
use std::sync::{
  mpsc::{Sender, self},
  Arc,
  Mutex};

use super::Worker;
use super::worker::Message;

pub struct ThreadPool {
  pub sender: Sender<Message>,
  pub workers: Vec<Worker>,
}

impl ThreadPool {
  pub fn new(size: usize) -> Self {
    assert!(size > 0);

    let (sender, receiver)
      = mpsc::channel();
    let receiver
      = Arc::new(Mutex::new(receiver));

    let mut pool = Vec::with_capacity(size);

    for id in 0..size {
      pool.push(Worker::new(id, Arc::clone(&receiver)));
    }

    Self {
      workers: pool,
      sender,
    }
  }

  pub fn exceute<F>(&self, f: F)
  where F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}
