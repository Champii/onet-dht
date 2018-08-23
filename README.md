# Rust-DHT

## Usage

```rust
use rust_dht::*;

fn main() {
  let config: DhtConfig = Default::default();

  let mut dht = Dht::new(config);

  dht.run();

  let hash = dht.store("test".as_bytes().to_vec());

  let res = dht.fetch(hash.clone()).unwrap();

  println!("Hash {}, Res {}", hash, String::from_utf8(res).unwrap());

  dht.close();
}
```