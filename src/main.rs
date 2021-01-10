use std::env;
use std::fs::File;
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

use i3status_bar::*;

// use chrono;
// use handlebars;

fn main() -> Result<(), anyhow::Error> {
  let args: Vec<_> = env::args().collect();
  let file_name = if args.len() < 2 {
    "i3status.conf"
  } else {
    &args[1]
  };
  // Panic if config file can't be opened
  let f = File::open(file_name).unwrap();

  let conf: Config = serde_yaml::from_reader(f)?;
  // Print out header for the bar 
  // Write '\n[[]' to make things easier to start with
  println!("{}\n[\n[]", serde_json::to_string(&conf.header)?);
  // Convert the config into the correct i3 format that we can modify
  let mut blocks : Vec<Field> = conf.blocks.into_iter().flat_map(|m|
    m.into_iter().map(|(k, v)| 
        Field { 
          name: Some(k), 
          ..v
        }
    )
  ).collect();
  
  // Loop forever to print out the next bar
  loop {
    let start = Instant::now();
    for block in &mut blocks {
      // Check for commands right now
      if let Some(c) = &block.command {
        if c.len() > 0 {
          let mut cmd = Command::new(&c[0]);
          block.full_text = Some(String::from_utf8(
            cmd.args(&c[1..]).output()?.stdout
          )?)
        }
      }
    }
    // Print out our blocks
    println!(",{}", serde_json::to_string(&blocks)?);
    // Sleep for up to a second
    sleep(Duration::from_secs(1) - (Instant::now() - start));
  }
  Ok(())
}
