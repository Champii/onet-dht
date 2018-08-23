use shrust::{Shell, ShellIO};
use std::io::prelude::*;
use std::thread;
use super::Dht;

pub fn run(dht: Dht) {
  // let mut dht = dht.clone();

  thread::spawn(move || {
    let mut shell = Shell::new(dht);

    shell.new_command("s", "Store", 1, |io, dht, cmd| {
      let res = dht.store(cmd[0].as_bytes().to_vec());

      try!(writeln!(io, "Stored: {}", res));

      Ok(())
    });

    shell.new_command("f", "Fetch", 1, |io, dht, cmd| {
      let res = dht.fetch(cmd[0].to_string());

      try!(writeln!(io, "Fetch: {:?}", res));

      Ok(())
    });

    shell.new_command_noargs("r", "Routing", |io, dht| {
      for (h, _) in dht.routing.get().buckets.iter() {
        try!(writeln!(io, "{}", h));
      }

      Ok(())
    });

    shell.new_command_noargs("l", "List Storage", |io, dht| {
      writeln!(io, "Size: {}o", dht.storage.get().size());

      for (h, d) in dht.storage.get().store.iter() {
        try!(writeln!(io, "{}, {}o", h, d.len()));
      }

      Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
  });
}