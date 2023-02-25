
use std::thread::{self, JoinHandle};
use std::sync::{
  mpsc::Receiver,
  Arc,
  Mutex,
};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
  NewJob(Job),
  Terminate,
}

pub struct Worker {
  pub id: usize,
  pub thread: Option<JoinHandle<()>>,
}

impl Worker {

  pub fn new(id: usize, reciver: Arc<Mutex<Receiver<Message>>>) -> Self {
    let thread = thread::spawn(move || loop {
      let message = reciver.lock().unwrap().recv().unwrap();

      match message {
        Message::NewJob(job) => {
          job();
        },
        Message::Terminate => break,
      }
    });

    Self {
      id,
      thread: Some(thread),
    }
  }
}
