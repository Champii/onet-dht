extern crate hex;

use std::io::Result;
use std::collections::HashMap;
use xor::xor;
use std::str;
use std::cmp::Ordering::*;

use super::node::*;

const MAX_BUCKET_LEN: usize = 4;

#[derive(Clone, Default, Debug)]
pub struct Routing {
  pub hash: String,
  pub buckets: HashMap<String, Node>,
}

impl Routing {
  pub fn new(hash: String) -> Routing {
    Routing{
      hash,
      buckets: HashMap::new(),
    }
  }

  pub fn try_add(&mut self, node: Node) -> Result<()> {
    let hash = node.hash.clone();
    let farthest = self.get_farthest();

    if self.buckets.len() >= MAX_BUCKET_LEN - 1 && self.is_farther(&hash, &farthest.clone().unwrap().hash) {
      trace!("Node is too far, ignoring {}", hash);

      return Ok(())
    }

    if let None = self.buckets.get(&hash) {

      info!("Inserting {} ({})", hash, self.buckets.len() + 1);
      self.buckets.insert(hash, node);

      if self.buckets.len() >= MAX_BUCKET_LEN {
        debug!("Bucket full ! Removing farthest");

        self.remove(&farthest.unwrap().hash);
      }
    }

    Ok(())
  }

  pub fn get_farthest(&mut self) -> Option<Node> {
    let mut max = &self.hash;
    let mut node = None;

    for (hash, node_) in self.buckets.iter() {
      if self.is_farther(&hash, max) {
        max = hash;

        node = Some(node_.clone());
      }
    }

    node
  }

  pub fn get_nearest_of(&self, h: String) -> Option<Node> {
    let mut min = &self.hash;
    let mut node = None;

    for (hash, node_) in self.buckets.iter() {
      if self.is_nearer_of(&hash, &min, &h) {
        min = hash;

        node = Some(node_.clone());
      }
    }

    node
  }

  pub fn is_farther(&self, h1: &String, h2: &String) -> bool {
    let dist1 = Self::xor_distance(&self.hash, h1);
    let dist2 = Self::xor_distance(&self.hash, h2);

    if let Greater = dist1.cmp(&dist2) {
      true
    } else {
      false
    }
  }

  #[allow(unused)]
  pub fn is_nearer(&self, h1: &String, h2: &String) -> bool {
    let dist1 = Self::xor_distance(&self.hash, h1);
    let dist2 = Self::xor_distance(&self.hash, h2);

    if let Less = dist1.cmp(&dist2) {
      true
    } else {
      false
    }
  }

  pub fn is_nearer_of(&self, h1: &String, h2: &String, of: &String) -> bool {
    let dist1 = Self::xor_distance(of, h1);
    let dist2 = Self::xor_distance(of, h2);

    if let Less = dist1.cmp(&dist2) {
      true
    } else {
      false
    }
  }

  pub fn remove(&mut self, hash: &String) {
    if hash != "" {
      debug!("Removing {}", hash);

      self.buckets.remove(hash).unwrap();
    }
  }

  pub fn xor_distance(s1: &String, s2: &String) -> String {
    let a1 = &s1.as_bytes();
    let a2 = &s2.as_bytes();

    let result = xor(a1, a2);

    if let Ok(res) = str::from_utf8(&result) {
      return hex::encode(String::from(res));
    }

    String::from("")
  }

  // pub fn get_nearest_bucket(&self, hash: &String) -> Vec<Node> {

  // }
}
