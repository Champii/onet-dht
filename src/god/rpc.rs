use rsrpc::*;

type Hash = String;
type Data = Vec<u8>;

service! {
  rpc ping() ->  bool;
  rpc fetch(hash: Hash) -> Data;
  rpc fetch_node(hash: Hash) -> Vec<Hash>;
  rpc store(data: Data) -> Hash;
}

pub struct DhtService;

impl Service for DhtService {
  fn ping() -> bool {
    trace!("> Ping");
    trace!("< Pong");
    true
  }

  fn fetch(hash: Hash) -> Data {
    trace!("> Fetch");
    trace!("< Found");
    Vec::new()
  }

  fn fetch_node(hash: Hash) -> Vec<Hash> {
    trace!("> Fetch Node");
    trace!("> Found Node");
    Vec::new()
  }

  fn store(data: Data) -> Hash {
    trace!("> Store");
    trace!("< Stored");
    Hash::new()
  }
}
