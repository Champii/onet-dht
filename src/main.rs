#![feature(
  async_await,
  await_macro,
  pin,
  arbitrary_self_types,
  futures_api
)]
#[macro_use]
extern crate log;
extern crate clap;
extern crate rust_dht;

mod args;

use rust_dht::logger;
pub use rust_dht::Dht;

fn main() {
  let config = args::parse_config();

  logger::init_logger(config.verbose);

  let mut dht = Dht::new(config);

  dht.run(true);

  dht.wait_close();
}
