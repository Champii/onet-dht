use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Node {
  pub addr: SocketAddr,
  pub hash: String,
}

impl Node {
  pub fn new(addr: SocketAddr, hash: String) -> Node {
    Node {
      addr,
      hash,
    }
  }
}
