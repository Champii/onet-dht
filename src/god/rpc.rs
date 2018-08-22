type Hash = String;
type Data = Vec<u8>;
type Nodes = Vec<Hash>;

service! {
  Rpc {
    fn ping() -> bool {
      true
    }

    fn fetch(hash: String) -> Vec<u8> {
      Vec::new()
    }

    fn fetch_node(hash: String) -> Vec<String> {
      Vec::new()
    }

    fn store(data: Vec<u8>) -> String {
      String::new()
    }
  }
}
