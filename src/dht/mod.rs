extern crate rand;
extern crate hex;

use std::io::{ Result };
use std::net::{ SocketAddr };
use std::sync::{ Arc };
use rsrpc::{ Wrapper, Packet, Plugins };
use sha2::{ Sha256, Digest };
use std::fmt::{ Debug, Result as FResult, Formatter };
use std::thread;

mod routing;
mod storage;
// mod proto;
pub mod args;
mod node;
mod cli;
mod rpc;
mod utils;
mod logger;

use self::routing::*;
use self::storage::*;
use self::node::*;
use self::rpc::*;
use self::utils::*;

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
  hash: String,
  routing: Mutexed<Routing>,
  storage: Mutexed<Storage>,
  config: DhtConfig,
  handle: Option<Arc<thread::JoinHandle<()>>>,
}

struct HashWrapper {
  hash: String,
  routing: Mutexed<Routing>,
}

impl Wrapper for HashWrapper {
  fn on_send(&self, pack: &Packet) -> Packet {
    let mut res = self.hash.clone().as_bytes().to_vec();
    let mut mut_pack = pack.clone();

    res.append(&mut pack.data.clone());

    mut_pack.data = res.clone();

    mut_pack
  }

  fn on_recv(&self, pack: &Packet) -> Packet {
    let mut mut_pack = pack.clone();

    let mut d = pack.data.clone();

    let data = d.split_off(self.hash.len());

    let hash = String::from_utf8(d).unwrap();

    let sender = pack.header.sender.clone();

    self.routing.map(Box::new(move |routing: &mut Routing| {
      let hash_cpy = hash.clone();

      routing.try_add(Node::new(sender, hash_cpy)).unwrap();
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

  pub fn run(&mut self) {
    logger::init_logger(self.config.verbose);

    info!("Hash is {}", self.hash);

    Plugins::add(HashWrapper{
      hash: self.hash.clone(),
      routing: self.routing.clone(),
    });

    self.bootstrap().unwrap();

    let addr = self.config.listen_addr.clone().to_string();

    let server = Rpc::listen(&addr);

    {
      let mut guard = server.context.lock().unwrap();
      (*guard).dht = self.clone();
    }

    cli::run(self.clone());

    self.handle = Some(Arc::new(thread::spawn(move || {
      Rpc::Server::wait_thread(server);
    })));
  }

  fn bootstrap(&self) -> Result<()> {
    if let Some(addr) = self.config.connect_addr {
      trace!("Connecting to {}", addr);

      let mut bootstrap_node = Rpc::connect(&self.config.connect_addr.unwrap().to_string());

      let res = bootstrap_node.ping();
      debug!("Connected: {}", res);
    } else {
      warn!("Bootstrap node !");
    }

    Ok(())
  }

  pub fn store(&mut self, data: Vec<u8>) -> String {
    let hash = Self::make_hash(data.clone());

    match self.routing.get().get_nearest_of(hash.clone()) {
      Some(node) => {
        let mut client = Rpc::connect(&node.addr.to_string());

        client.store(data)
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
          let mut client = Rpc::connect(&node.addr.to_string());

          client.fetch(hash)
        } else {
          None
        }
      }
      Some(res) => {
        Some(res)
      }
    }
  }

  pub fn wait_close(&mut self) {
    if let Some(handle) = self.handle.take() {
      let mut handle = Arc::try_unwrap(handle).unwrap();

      handle.join().unwrap();
    }
  }
}

impl Default for Dht {
  fn default() -> Dht {
    Dht::new(Default::default())
  }
}