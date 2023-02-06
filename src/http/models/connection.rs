
use std::num::TryFromIntError;
use std::panic;

fn str_to_u8(s: &str) -> u8 {
  let i = s.parse::<i32>();
  let r: Result<u8, TryFromIntError>;
  match i {
    Ok(n) => r = n.try_into(),
    Err(_e) => panic!("could not parse part of the input as an i32"),
  }

  match r {
    Ok(n) => n,
    Err(_e) => panic!("could not parse part of the input as an u8"),
  }
}

pub struct Connection {
  ip: [u8; 4],
  port: u16,
}

impl Connection {

  pub fn from_string(ipv4: impl Into<String>) -> Self {
    let binding = ipv4.into();
    let mut split = binding.split(":");
    let (ip, port): ([u8; 4], u16) = (Self::parse_ip(split.next().unwrap()), Self::parse_port(split.next().unwrap()));

    Self { ip, port }
  }

  pub fn get_ip(&self) -> &[u8; 4] {
    &self.ip
  }

  pub fn get_port(&self) -> &u16 {
    &self.port
  }

  fn parse_ip(ip: impl Into<String>) -> [u8; 4] {
    todo!();
  }

  fn parse_port(port: impl Into<String>) -> u16 {
    todo!();
  }

}
