extern crate hex;

use std::io::Result;
use std::collections::HashMap;
use xor::xor;
use std::str;
use std::cmp::Ordering::*;

use super::node::*;

const MAX_BUCKET_LEN: usize = 4;

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
    let hash = node.sender.hash.clone();
    let farthest = self.get_farthest();

    if self.buckets.len() >= MAX_BUCKET_LEN - 1 && self.is_farther(&hash, &farthest) {
      println!("Dont add: farther");

      return Ok(())
    }

    if let None = self.buckets.get(&hash) {
      println!("Insert");
      self.buckets.insert(hash, node);

      if self.buckets.len() >= MAX_BUCKET_LEN {
        println!("Removed farthest");
        self.remove(&farthest);
      }
    }

    Ok(())
  }

  pub fn get_farthest(&mut self) -> String {
    let mut max = &String::from("");

    for (hash, _) in self.buckets.iter() {
      let dist = Self::xor_distance(&hash, &self.hash);

      if self.is_farther(&dist, max) {
        max = hash;
      }
    }

    max.clone()
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

  pub fn remove(&mut self, hash: &String) {
    if hash != "" {
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
}
