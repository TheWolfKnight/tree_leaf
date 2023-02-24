#![allow(unused)]

mod services;
mod models;

use std::io::{Error, self, Read};
use std::net::{TcpListener, TcpStream};
use std::str;

fn handle_clinet(stream: Result<TcpStream, Error>) -> () {
    println!("Got stream");
    match stream {
        Ok(mut content) => {
            let mut stream_buffer: [u8; 512] = [0; 512];
            let result: Result<usize, Error> = content.read(&mut stream_buffer);

            if let Ok(_) = result {
                println!("{}", str::from_utf8(&stream_buffer).unwrap());
            }
        },
        Err(_) => println!("Could not get that"),
    }
}

fn main() {
    let listener: io::Result<TcpListener> = TcpListener::bind("127.0.0.1:6942");

    match listener {
        Ok(list) => {
            println!("Connected");
            for stream in list.incoming() {
                handle_clinet(stream);
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
