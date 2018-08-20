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

use std::io::Result;

mod god;

use god::God;

fn main() -> Result<()> {
  env_logger::init();

  God::new().run()
}
