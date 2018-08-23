type Hash = String;
type Data = Vec<u8>;
type Nodes = Vec<Hash>;

use super::Routing;
use super::Storage;
use super::utils::*;
use super::Dht;
use std::collections::HashMap;
use std::sync::Arc;

service! {
  Rpc {
    let dht: super::Dht;

    fn ping(&mut self,) -> bool {
      true
    }

    fn fetch(&mut self, hash: super::Hash) -> Option<Vec<u8>> {
      self.dht.fetch(hash)
    }

    fn store(&mut self, data: Vec<u8>) -> String {
      self.dht.store(data.clone())
    }
  }
}
