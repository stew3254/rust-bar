use std::env;
use std::fs::File;
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

use i3status_bar::*;
use chrono::prelude::*;
use sysinfo::{SystemExt, DiskExt, ProcessorExt};
// use handlebars;

fn read_config(file_name: &str) -> Result<(Module, Vec<Field>), anyhow::Error> {
  // Panic if config file can't be opened or read correctly
  let f = File::open(file_name).unwrap();
  let conf: Config = serde_yaml::from_reader(f)?;

  // Print out header for the bar
  // Write '\n[[]' to make things easier to start with
  println!("{}\n[\n[]", serde_json::to_string(&conf.i3.header)?);

  // Convert the config into the correct i3 format that we can modify
  let blocks: Vec<Field> = conf.i3.blocks.into_iter().flat_map(|m|
    m.into_iter().map(|(k, v)|
      Field {
        name: Some(k.to_ascii_lowercase()),
        ..v
      }
    )
  ).collect();

  let mut module = Module::new();

  for module_type in conf.builtin {
    match module_type {
      ModuleType::Components(m) => module.components = Some(m),
      ModuleType::CPUs(m) => module.cpus = Some(m),
      ModuleType::Disks(m) => module.disks = Some(m),
      ModuleType::Memory(m) => module.memory = Some(m),
      ModuleType::Net(m) => module.netifaces = Some(m),
      ModuleType::Processes(m) => module.processes = Some(m),
      ModuleType::System(m) => module.system = Some(m),
      ModuleType::Users(m) => module.users = Some(m),
    }
  }

  // Return the modules config and the blocks
  Ok((module, blocks))
}

fn gen_closures() -> Vec<Box<dyn FnMut(&mut Field)>> {
  let mut closures: Vec<Box<dyn FnMut(&mut Field)>> = Vec::new();
  closures.push(Box::new(|c| {
    let mut sys = sysinfo::System::new();
    do_cpu(&mut sys, c);
  }));
  // closures.push(|c| {
  //   let mut sys = System::new();
  //   do_cpu(&mut sys, c);
  // });
  closures
}

fn do_time(block: &mut Field) {
  let t = Local::now();
  // If no normal format found, just do 24 hour time
  block.full_text =  match &block.full_text_format {
    Some(f) => Some(t.format(f).to_string()),
    None => Some(t.format("%H:%M:%S").to_string()),
  };

  // If no short format found, ignore it
  block.short_text =  match &block.short_text_format {
    Some(f) => Some(t.format(f).to_string()),
    None => None,
  };
}

fn do_cpu(sys: &mut sysinfo::System, block: &mut Field) {
  sys.refresh_cpu();
  println!("Here");
}

fn main() -> Result<(), anyhow::Error> {
  // Get program args
  let args: Vec<_> = env::args().collect();
  let file_name = if args.len() < 2 {
    "i3status.conf"
  } else {
    &args[1]
  };

  // Get block configuration
  let (module, mut blocks) = read_config(file_name)?;
  
  // Loop forever to print out the next bar
  loop {
    let start = Instant::now();
    for block in &mut blocks {
      if let Some(name) = &block.name {
        // Handle all built-in block features
        if let Some(_) = &block.builtin {
          match name.as_str() {
            "time" => {
              do_time(block);
            },
            "cpu" => {
              // sys.refresh_disks();
              // for x in sys.get_disks() {
              // }
            }
            _ => {}
          }
        }
      }

      // Check if it's a command
      if let Some(c) = &block.command {
        if c.len() > 0 {
          let mut cmd = Command::new(&c[0]);
          block.full_text = Some(String::from_utf8(cmd.args(&c[1..]).output()?.stdout)?)
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
