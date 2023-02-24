
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::io;

use super::WebPath;

pub struct Server {
  pub connection: Ipv4Addr,
  pub web_path: WebPath,
}

impl Server {
  pub fn new(connection: impl Into<Ipv4Addr>, local_path: impl Into<String>) -> Self {
    let local_path: String = local_path.into();
    let connection: Ipv4Addr = connection.into();
    Self {
      connection,
      web_path: WebPath::new(local_path)
    }
  }

  pub fn listen() -> io::Result<()> {
    todo!();
  }

}
