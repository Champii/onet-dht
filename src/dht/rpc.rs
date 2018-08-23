use super::Dht;

service! {
  Rpc {
    let dht: super::Dht;

    fn ping(&mut self,) -> bool {
      true
    }

    fn fetch(&mut self, hash: String) -> Option<Vec<u8>> {
      self.dht.fetch(hash)
    }

    fn store(&mut self, data: Vec<u8>) -> String {
      self.dht.store(data.clone())
    }
  }
}
