#![allow(unused)]


use std::{
  io::{Error, Read, Write, self},
  net::{TcpListener, TcpStream, self},
  process::exit,
  sync::{Arc, Mutex, MutexGuard},
  env,
  str,
};

mod services;
mod models;

use models::{Settings, ThreadPool, FolderNode};

fn read_file_to_end(buffer: &mut [u8]) -> usize {
  todo!();
}

fn handle_conncetion<'a>(mut stream: TcpStream, folder_tree: &'a Arc<Mutex<FolderNode>>) {
  println!("handle_conncetion");

  let folder_tree_locked = folder_tree.lock();

  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";

  if let Err(_e) = &folder_tree_locked {
    println!("Could net get tree");
  } 

  let inner_tree: MutexGuard<FolderNode> = folder_tree_locked.ok().unwrap();

  let contains_index: bool = inner_tree.contains_file(&["index.html"]);
  
  let (status_line, filename) = if buffer.starts_with(get) && contains_index {
    ("HTTP/1.1 200 OK", "<html><body><h1>Hello world</h1></body></html>")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "<html><body><h1>Page Not Found</h1></body></html>")
  };

  let content = filename;

  let response = format!(
    "{}\r\nContent-Length: {}\r\n\r\n{}",
    status_line,
    content.len(),
    content,
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn setup<'a>() -> Settings<'a> {
  let mut args: Vec<String> = env::args().rev().collect();
  let mut result = Settings::new();

  return result;

  args.pop();
  let mut count: i32 = 0;

  loop {
    let com = args.pop();
    match com {
      Some(c) => {
      },
      None => break,
    }
    count += 1;
  }

  if count == 0 {
    panic!("Could not finde an execution mode");
  }

  return result;
}

fn main() {
  let settings: Settings = setup();

  println!("settings done");

  let listener = TcpListener::bind(settings.target).unwrap();
  let pool = ThreadPool::new(settings.worker_pool_size);

  println!("building tree");
  let mut folder_tree: Arc<Mutex<FolderNode>> = Arc::new(Mutex::new(FolderNode::new(settings.prefix)));
  println!("done with tree");

  for stream in listener.incoming() {
    println!("stream listener");
    let stream = stream.unwrap();

    let input = handle_conncetion(stream, &folder_tree);
    pool.exceute(move || {
      input
    });
  }
}

#[cfg(test)]
mod tests {
  use crate::services::WebPath;
  use crate::services::Server;
  use crate::models::FolderNode;
  #[test]
  fn web_server_path_parse() {
    let prefix: &str = "/home/www/www.test.com/";
    let web_path: WebPath = WebPath::new(prefix);
    let web_path_result: Option<Vec<&str>> = web_path.parse_path("/home/www/www.test.com/hello/world.html");
    assert_eq!(web_path_result, Some(vec!["hello", "world.html"]));
  }
  #[test]
  fn server_ipv4_parsing() {
    let prefix: &str = "/home/www/www.test.com/";
    let server: Server = Server::new([127, 0, 0, 1], prefix);
    assert_eq!(Ok(server.connection), "127.0.0.1".parse())
  }
   #[test]
  fn folder_tree_instance() {
    let path: &str = "C:/";
    let folder_tree: FolderNode = FolderNode::new(path);
    assert_eq!(folder_tree.name,       path);
    assert_eq!(folder_tree.files.len(),   0);
    assert_eq!(folder_tree.folders.len(), 0);
   }
}
