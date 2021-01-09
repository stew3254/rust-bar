use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
// use chrono;
// use handlebars;

#[derive(Debug, Serialize, Deserialize)]
struct Header {
  version: u32,
  stop_signal: Option<u16>,
  cont_signal: Option<u16>,
  click_events: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Field {
  full_text: Option<String>,
  short_text: Option<String>,
  command: Option<String>, // Custom, not a part of i3 protocol
  color: Option<String>,
  background: Option<String>,
  border: Option<String>,
  border_top: Option<u32>,
  border_right: Option<u32>,
  border_bottom: Option<u32>,
  border_left: Option<u32>,
  min_width: Option<u32>,
  align: Option<String>,
  name: Option<String>,
  instance: Option<String>,
  urgent: Option<bool>,
  separator: Option<bool>,
  separator_block_width: Option<u32>,
  markup: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
  header: Header,
  blocks: HashMap<String, Field>,
}

fn main() -> Result<(), anyhow::Error> {
  let args: Vec<_> = env::args().collect();
  let file_name = if args.len() == 0 {
    "i3status.conf"
  } else {
    &args[1]
  };
  let f = File::open(file_name).unwrap();

  let conf: Config = serde_yaml::from_reader(f)?;
  // Write '\n[[]' to make things easier to start wtih
  print!("{}\n[[]", serde_json::to_string(&conf.header)?);
  loop {
    sleep(Duration::from_millis(1000));
    println!("Hi");
  }

  Ok(())
  // for  in docs.iter().flat_map(|d| d.as_hash()) { }
}
