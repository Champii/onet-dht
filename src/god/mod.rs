#![feature(async_await, await_macro)]
extern crate rand;
extern crate hex;

use std::io::{ Result };
use std::net::{ SocketAddr };

mod routing;
mod proto;
mod args;
mod node;
mod rpc;

use self::routing::*;
use self::node::*;
use self::rpc::*;

pub struct GodConfig {
  pub verbose: u8,
  pub listen_addr: SocketAddr,
  pub connect_addr: Option<SocketAddr>,
}

pub struct God {
  hash: String,
  routing: Routing,
  config: GodConfig,
}


impl God {
  pub fn new() -> God {

    let config = args::parse_config();
    let hash = Self::new_hash();

    info!("Hash is {}", hash);

    God {
      routing: Routing::new(hash.clone()),
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

  pub fn run(&mut self) -> Result<()> {
    self.bootstrap()?;

    let addr = self.config.listen_addr.clone().to_string();

    Rpc::Server::wait_thread(Rpc::listen(&addr));

    Ok(())
  }

  pub fn bootstrap(&self) -> Result<()> {
    if let Some(addr) = self.config.connect_addr {
      info!("Connecting to {}", addr);

      let mut bootstrap_node = Rpc::connect(&self.config.connect_addr.unwrap().to_string());

      // self.routing.try_add(Node {
      //   sender: Sender {

      //   }
      // });

      info!("Connected: {}", bootstrap_node.ping());
    } else {
      info!("Bootstrap node !");
    }

    Ok(())
  }
}
