use std::net::SocketAddr;

use super::God;
use super::Node;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sender {
  pub addr: SocketAddr,
  pub hash: String,
}

impl Sender {
  pub fn to_node(&self) -> Node {
    Node {
      sender: Sender {
          hash: self.hash.clone(),
          addr: self.addr.clone(),
      },
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PacketHeader {
  pub sender: Sender,
  pub date: u64,
  pub msg_hash: String,
  pub response_to: String,
}

impl PacketHeader {
  pub fn new(god: &God, response_to: String) -> PacketHeader {
    PacketHeader {
      sender: Sender {
        addr: god.config.listen_addr,
        hash: god.hash.clone(),
      },
      date: 0,
      msg_hash: String::from(""),
      response_to,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketData {
  Ping,
  Pong,
  Fetch(String),
  FetchNode(String),
  Store(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
  pub header: PacketHeader,
  pub data: PacketData,
}

impl Packet {
  pub fn new_ping(god: &God) -> Packet {
    Packet {
      header: PacketHeader::new(god, String::from("")),
      data: PacketData::Ping,
    }
  }

  pub fn new_pong(god: &God, response_to: String) -> Packet {
    Packet {
      header: PacketHeader::new(god, response_to),
      data: PacketData::Pong,
    }
  }
}
