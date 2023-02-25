#![allow(unused)]

use std::{
  io::{Error, Read, Write, self},
  net::{TcpListener, TcpStream, self},
  process::exit,
  sync::Mutex,
  str
};

mod services;
mod models;

use models::ThreadPool;

fn handle_conncetion(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", "<html><body><h1>Hello, world!</h1></body></html>")
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


fn main() {
  let listener = TcpListener::bind("127.0.0.1:6942").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.exceute(|| {
      handle_conncetion(stream);
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
