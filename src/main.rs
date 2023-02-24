#![allow(unused)]

mod services;
mod models;

use std::{
  io::{Error, Read, Write, self},
  net::{TcpListener, TcpStream, self},
  process::exit,
  sync::Mutex,
  str,
};

fn handle_conncetion(stream: &Mutex<TcpStream>) -> () {
  println!("got stream");
  let mut stream = stream.lock().unwrap();
  let mut stream_buffer: [u8; 512] = [0; 512];
  let read: io::Result<usize> = stream.read(&mut stream_buffer);

  match read {
    Ok(_) => println!("{}", str::from_utf8(&stream_buffer).unwrap()),
    Err(_) => println!("Could not read"),
  }

  stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body><h1>Hello world!</h1></body></html>");
  let send: io::Result<()> = stream.flush();

  match send {
    Ok(_) => println!("Bytes send"),
    Err(_) => println!("Could not send bytes"),
  }
}

fn main() {
  let listener: io::Result<TcpListener> = TcpListener::bind("127.0.0.1:6942");
  match listener {
  Ok(list) => {
    println!("Connected");
    for stream in list.incoming() {
      match stream {
        Ok(mut tcp_stream) => {
          let protected_strem: Mutex<TcpStream> = Mutex::new(tcp_stream);
          handle_conncetion(&protected_strem);
        },
        Err(_) => todo!(),
      }
    }
  },
    Err(_) => println!("Could not connect!"),
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
