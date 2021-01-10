use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
  pub version: u32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_signal: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cont_signal: Option<u16>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub click_events: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
  pub full_text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub short_text: Option<String>,
  // Custom, not a part of i3 protocol
  #[serde(skip_serializing)]
  pub command: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub color: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub background: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub border: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub border_top: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub border_right: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub border_bottom: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub border_left: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub min_width: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub align: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub instance: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub urgent: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub separator: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub separator_block_width: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub markup: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub header: Header,
  pub blocks: Vec<HashMap<String, Field>>,
}