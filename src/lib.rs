use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
  pub version: u32,
  pub stop_signal: Option<u16>,
  pub cont_signal: Option<u16>,
  pub click_events: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
  pub full_text: Option<String>,
  pub short_text: Option<String>,
  pub color: Option<String>,
  pub background: Option<String>,
  pub border: Option<String>,
  pub border_top: Option<u32>,
  pub border_right: Option<u32>,
  pub border_bottom: Option<u32>,
  pub border_left: Option<u32>,
  pub min_width: Option<u32>,
  pub align: Option<String>,
  pub name: Option<String>,
  pub instance: Option<String>,
  pub urgent: Option<bool>,
  pub separator: Option<bool>,
  pub separator_block_width: Option<u32>,
  pub markup: Option<String>,

  // Custom fields that aren't part of the i3 protocol
  pub builtin: Option<bool>,
  pub command: Option<Vec<String>>,
  pub full_text_format: Option<String>,
  pub short_text_format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct I3Config {
  pub header: Header,
  pub blocks: Vec<HashMap<String, Field>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
  pub label: Option<String>,
  pub temp: Option<char>,
  pub critical_temp: Option<char>,
  pub max_temp: Option<char>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CPU {
  pub name: Option<String>,
  pub frequency: Option<String>,
  pub usage: Option<String>,
  pub vendor_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disk {
  pub name: Option<String>,
  pub avail: Option<char>,
  pub used: Option<char>,
  pub free: Option<char>,
  pub total: Option<char>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
  pub avail: Option<char>,
  pub used: Option<char>,
  pub free: Option<char>,
  pub total: Option<char>,
  pub avail_swap: Option<char>,
  pub used_swap: Option<char>,
  pub free_swap: Option<char>,
  pub total_swap: Option<char>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetIface {
  pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Process {
  pub name: Option<String>,
  pub cmd: Option<Vec<String>>,
  pub pid: Option<u64>,
  pub memory_used: Option<char>,
  pub virtual_used: Option<char>,
  pub start_time: Option<char>,
  pub cpu_usage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub name: Option<String>,
  pub groups: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct System {
  pub uptime: Option<char>,
  pub boot_time: Option<char>,
  pub load_avg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
  pub components: Option<Vec<Component>>,
  pub cpus: Option<Vec<CPU>>,
  pub disks: Option<Vec<Disk>>,
  pub memory: Option<Memory>,
  pub netifaces: Option<Vec<NetIface>>,
  pub processes: Option<Vec<Process>>,
  pub system: Option<System>,
  pub users: Option<Vec<User>>,
}

impl Module {
  pub fn new() -> Module {
    Module {
      components: None,
      cpus: None,
      disks: None,
      memory: None,
      netifaces: None,
      processes: None,
      system: None,
      users: None
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModuleType {
  #[serde(alias = "components")]
  Components(Vec<Component>),
  #[serde(alias = "cpus")]
  CPUs(Vec<CPU>),
  #[serde(alias = "disks")]
  Disks(Vec<Disk>),
  #[serde(alias = "memory")]
  Memory(Memory),
  #[serde(alias = "net")]
  Net(Vec<NetIface>),
  #[serde(alias = "processes")]
  Processes(Vec<Process>),
  #[serde(alias = "system")]
  System(System),
  #[serde(alias = "users")]
  Users(Vec<User>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub builtin: Vec<ModuleType>,
  pub i3: I3Config,
}
