use super::Mutexed;
use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub struct Storage {
  pub store: HashMap<String, Vec<u8>>,
}

impl Storage {
  pub fn new() -> Storage {
    Storage {
      store: HashMap::new(),
    }
  }

  pub fn get(&self, hash: String) -> Option<Vec<u8>> {
    match self.store.get(&hash) {
      None => None,
      Some(vec) => Some(vec.clone()),
    }
  }

  pub fn add(&mut self, hash: String, data: Vec<u8>) {
    let size = self.size();

    self.store.insert(hash.clone(), data.clone());

    info!("Adding to storage {} ({}o, {} items)", hash, size + data.len(), self.store.len());
  }

  pub fn size(&self) -> usize {
    let mut acc = 0;

    for (_, v) in self.store.iter() {
      acc = acc + v.len();
    }

    acc
  }

}
