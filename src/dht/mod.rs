#[macro_use]
extern crate log;
#[macro_use]
extern crate rsrpc;
#[macro_use]
pub extern crate lazy_static;

extern crate hex;
extern crate rand;

extern crate bincode;
extern crate env_logger;
extern crate futures;
extern crate ring;
extern crate serde;
extern crate serde_bytes;
extern crate sha2;
extern crate shrust;
extern crate untrusted;
extern crate xor;

pub use rsrpc::{Packet, Plugins, Wrapper};
use sha2::{Digest, Sha256};
use std::fmt::{Debug, Formatter, Result as FResult};
use std::io::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;

pub mod routing;
mod storage;
// mod proto;
mod cli;
mod key;
pub mod logger;
mod node;
mod rpc;
mod utils;

use self::key::Key;
pub use self::node::*;
use self::routing::*;
pub use self::rpc::*;
use self::storage::*;
pub use self::utils::*;

#[derive(Clone, Debug)]
pub struct DhtConfig {
  pub verbose: u8,
  pub listen_addr: SocketAddr,
  pub connect_addr: Option<SocketAddr>,
}

impl Default for DhtConfig {
  fn default() -> DhtConfig {
    DhtConfig {
      verbose: 2,
      listen_addr: "127.0.0.1:3000".parse().unwrap(),
      connect_addr: None,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Dht {
  pub hash: String,
  pub key: Key,
  pub routing: Mutexed<Routing>,
  storage: Mutexed<Storage>,
  config: DhtConfig,
  handle: Option<Arc<thread::JoinHandle<()>>>,
}

struct HashWrapper {
  hash: String,
  pub_key: Vec<u8>,
  routing: Mutexed<Routing>,
}

impl Wrapper for HashWrapper {
  fn on_send(&self, pack: &Packet) -> Packet {
    let mut hash = self.hash.clone().as_bytes().to_vec();

    let mut pub_key = self.pub_key.clone();

    let mut mut_pack = pack.clone();

    hash.append(&mut pub_key);
    hash.append(&mut pack.data.clone());

    mut_pack.data = hash.clone();

    mut_pack
  }

  fn on_recv(&self, pack: &Packet) -> Packet {
    let mut mut_pack = pack.clone();

    let mut d = pack.data.clone();

    // extract the hash
    let mut pub_key = d.split_off(self.hash.len());

    // extract the pubKey
    let data = pub_key.split_off(self.pub_key.len());

    let hash = String::from_utf8(d).unwrap();

    let sender = pack.header.sender.clone();

    self.routing.map(Box::new(move |routing: &mut Routing| {
      let hash_cpy = hash.clone();

      routing
        .try_add(Node::new(sender, hash_cpy, pub_key.clone()))
        .unwrap();
    }));

    mut_pack.data = data.clone().to_vec();

    mut_pack
  }
}

impl Debug for HashWrapper {
  fn fmt(&self, fmt: &mut Formatter) -> FResult {
    write!(fmt, "HashWrapper")
  }
}

impl Dht {
  pub fn new(config: DhtConfig) -> Dht {
    let hash = Self::new_hash();

    Dht {
      key: Key::new_generate().unwrap(),
      routing: Mutexed::new(Routing::new(hash.clone())),
      storage: Mutexed::new(Storage::new()),
      handle: None,
      hash,
      config,
    }
  }

  pub fn new_hash() -> String {
    let mut key: Vec<u8> = Vec::new();

    for _ in 0..20 {
      key.push(rand::random::<u8>());
    }

    hex::encode(key)
  }

  pub fn make_hash(data: Vec<u8>) -> String {
    let mut sha = Sha256::new();

    sha.input(data.as_slice());

    hex::encode(sha.result())
  }

  pub fn run(&mut self, bootstrap: bool) {
    info!("Hash is {}", self.hash);

    let addr = self.config.listen_addr.clone().to_string();

    let mut server = Rpc::Duplex::listen(&addr);

    server.network.plugins.add(HashWrapper {
      pub_key: self.key.get_pub(),
      hash: self.hash.clone(),
      routing: self.routing.clone(),
    });

    if bootstrap {
      self.bootstrap().unwrap();
    }

    {
      let mut guard = server.context.lock().unwrap();
      (*guard).dht = self.clone();
    }

    cli::run(self.clone());
  }

  pub fn bootstrap(&self) -> Result<()> {
    if let Some(addr) = self.config.connect_addr {
      trace!("Connecting to {}", addr);

      let mut bootstrap_node = Rpc::Duplex::connect(&self.config.connect_addr.unwrap().to_string());

      bootstrap_node.ping().unwrap().unwrap();

      debug!("Connected to : {}", addr);
    } else {
      warn!("Bootstrap node !");
    }

    Ok(())
  }

  pub fn store(&mut self, data: Vec<u8>) -> String {
    let hash = Self::make_hash(data.clone());

    match self.routing.get().get_nearest_of(hash.clone()) {
      Some(node) => {
        let mut client = Rpc::Duplex::connect(&node.addr.to_string());

        match client.store(data.clone()) {
          Ok(r) => match r {
            Ok(s) => s,
            _ => String::new(),
          },
          Err(_) => {
            self.routing.map(Box::new(move |routing: &mut Routing| {
              routing.remove(&node.hash)
            }));

            self.store(data)
          }
        }
      }
      None => {
        let hash2 = hash.clone();

        self.storage.map(Box::new(move |storage: &mut Storage| {
          storage.add(hash2.clone(), data.clone());
        }));

        hash
      }
    }
  }

  pub fn fetch(&mut self, hash: String) -> Option<Vec<u8>> {
    match self.storage.get().get(hash.clone()) {
      None => {
        if let Some(node) = self.routing.get().get_nearest_of(hash.clone()) {
          let mut client = Rpc::Duplex::connect(&node.addr.to_string());

          match client.fetch(hash.clone()) {
            Ok(r) => match r {
              Ok(s) => s,
              _ => None,
            },
            Err(_) => {
              self.routing.map(Box::new(move |routing: &mut Routing| {
                routing.remove(&node.hash)
              }));

              self.fetch(hash)
            }
          }
        } else {
          None
        }
      }
      Some(res) => Some(res),
    }
  }

  pub fn wait_close(&mut self) {
    Rpc::Duplex::wait()
  }
}

impl Default for Dht {
  fn default() -> Dht {
    Dht::new(Default::default())
  }
}
