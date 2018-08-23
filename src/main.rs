#![feature(async_await, await_macro, pin, arbitrary_self_types, futures_api)]
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate rsrpc;
extern crate env_logger;
extern crate serde;
extern crate serde_bytes;
extern crate bincode;
extern crate clap;
extern crate xor;
extern crate futures;
extern crate sha2;
extern crate shrust;

mod dht;

pub use dht::Dht;

fn main() {
  let config = dht::args::parse_config();

  let mut dht = Dht::new(config);

  dht.run();

  dht.wait_close();
}
