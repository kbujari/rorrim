use clap::{ArgAction::*, Parser, ValueEnum, ValueHint};

use serde::Deserialize;
use std::path::PathBuf;

pub const MIRROR_NUM: u8 = 10;
pub const MIRROR_URL: &str = "https://archlinux.org/mirrors/status/json/";

#[derive(Debug, Parser)]
#[command(author, version)]
pub struct Args {
    /// Custom URL used to fetch all mirrors
    #[arg(short, long, value_name = "URL", value_hint = ValueHint::Url)]
    pub url: Option<String>,

    /// Location to save mirrorlist to
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub save: Option<PathBuf>,

    /// Number of mirrors to output to file
    #[arg(short, long, value_name = "NUM")]
    pub number: Option<u16>,

    /// Protocols allowed to use
    #[arg(
        short,
        long,
        value_name = "http|https|ftp",
        value_enum,
        required = true
    )]
    pub protocol: Vec<Protocol>,

    /// Restrict mirrors to a set of countries or just one
    #[arg(short, long, value_name = "France", action = Append, required = true)]
    pub country: Vec<String>,

    /// Sort filtered mirrors by a metric
    #[arg(long, value_name = "age|score", value_enum)]
    pub sort: Option<String>,

    /// Do not use mirrors that serve ISOs
    #[arg(long, action = SetFalse)]
    pub no_iso: bool,

    /// Do not use mirrors that use IPv4
    #[arg(long, action = SetFalse)]
    pub no_ipv4: bool,

    /// Do not use mirrors that use IPv6
    #[arg(long, action = SetFalse)]
    pub no_ipv6: bool,

    /// Print additional information
    #[arg(long, action = SetTrue)]
    pub verbose: Option<bool>,
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Https,
    Http,
    Rsync,
}

impl Protocol {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Https => "https",
            Self::Http => "http",
            Self::Rsync => "rsync",
        }
    }
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    Age,
    Score,
}
