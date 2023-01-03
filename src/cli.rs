use clap::{
  Parser,
  ValueEnum,
  ArgAction::Append
};

use serde::Deserialize;
use std::path::PathBuf;

pub const MIRROR_NUM: u8 = 10;
pub const MIRROR_URL: &str = "https://archlinux.org/mirrors/status/json/";

#[derive(Debug, Parser)]
#[command(author, version)]
pub struct Args {
  /// Custom URL used to fetch all mirrors
  #[arg(short, long, value_name = "URL")]
  pub url: Option<String>,

  /// Location to save mirrorlist to
  #[arg(short, long, value_name = "FILE")]
  pub save: Option<PathBuf>,

  /// Number of mirrors to output to file
  #[arg(short, long, value_name = "NUM")]
  pub number: Option<u16>,

  /// Protocols allowed to use
  #[arg(short, long, value_name = "http|https|ftp", value_enum, )]
  pub protocol: Vec<Protocol>,

  /// Restrict mirrors to a set of countries by name or code
  #[arg(short, long, value_name = "Canada|CA", action = Append)]
  pub countries: Vec<String>,

  /// Sort filtered mirrors by a metric
  #[arg(long, value_name = "age|rate|score")]
  pub sort: Option<String>,

  /// Restrict to mirrors that host ISOs
  #[arg(long, action = clap::ArgAction::SetTrue)]
  pub isos: bool,

  /// Restrict to mirrors that support IPv4
  #[arg(long, action = clap::ArgAction::SetTrue)]
  pub ipv4: bool,

  /// Restrict to mirrors that support IPv6
  #[arg(long, action = clap::ArgAction::SetTrue)]
  pub ipv6: bool,

  /// Print additional information
  #[arg(long, action = clap::ArgAction::SetTrue)]
  pub verbose: Option<bool>,
}

#[derive(Debug, ValueEnum, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
  Https,
  Http,
  Rsync
}
