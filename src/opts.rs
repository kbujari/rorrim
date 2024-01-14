use clap::{ArgAction::*, ValueEnum, ValueHint};
use std::path::PathBuf;

/// CLI Arguments
#[derive(Debug, clap::Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Custom URL to download mirrorlist from, defaults to archlinux.org
    #[arg(short, long, value_name = "URL", value_hint = ValueHint::Url)]
    pub url: Option<String>,

    /// Path to save mirrorlist to
    #[arg(long, value_name = "PATH", value_hint = ValueHint::FilePath)]
    pub save: Option<PathBuf>,

    /// How many mirrors to keep in final output, defaults to 10
    #[arg(short, long, value_name = "NUM")]
    pub number: Option<usize>,

    /// Protocols used to connect to mirrors, defaults to ALL
    #[arg(short, long, value_enum)]
    pub protocol: Option<Vec<Protocol>>,

    /// Restrict to only mirrors in a country, defaults to ALL
    #[arg(short, long, value_name = "Country")]
    pub country: Option<String>,

    /// Sort filtered mirrors by their age since last sync or mirror "score"
    #[arg(short, long, value_enum)]
    pub sort: Option<Sort>,

    /// Don't use mirrors that also serve ISOs
    #[arg(long, action = SetFalse)]
    pub isos: bool,

    /// Disable mirrors that connect via IPv4
    #[arg(long, action = SetFalse)]
    pub ipv4: bool,

    /// Disable mirrors that connect via IPv6
    #[arg(long, action = SetFalse)]
    pub ipv6: bool,
}

/// Protocols used to download from mirror
#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum Protocol {
    Https,
    Http,
    Rsync,
}

impl std::str::FromStr for Protocol {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "https" => Ok(Self::Https),
            "http" => Ok(Self::Http),
            "rsync" => Ok(Self::Rsync),
            _ => Err("invalid protocol"),
        }
    }
}

/// How to sort filtered list of mirrors before output
#[derive(Debug, Clone, ValueEnum)]
pub enum Sort {
    Age,
    Score,
}
