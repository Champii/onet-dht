use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Node {
  pub addr: SocketAddr,
  pub hash: String,
  pub pub_key: Vec<u8>,
}

impl Node {
  pub fn new(addr: SocketAddr, hash: String, pub_key: Vec<u8>) -> Node {
    Node {
      addr,
      hash,
      pub_key,
    }
  }
}
