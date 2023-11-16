use clap::{ArgAction::*, ValueEnum, ValueHint};
use std::{fmt, path::PathBuf};

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

    /// Restrict to only mirrors in certain countries, defaults to ALL
    #[arg(short, long, value_name = "France", action = Append)]
    pub country: Option<Vec<String>>,

    /// Sort filtered mirrors by their age since last sync or mirror "score"
    #[arg(short, long, value_enum)]
    pub sort: Option<Sort>,

    /// Don't use mirrors that also serve ISOs
    #[arg(long, action = SetFalse)]
    pub no_iso: bool,

    /// Disable mirrors that connect via IPv4
    #[arg(long, action = SetFalse)]
    pub no_ipv4: bool,

    /// Disable mirrors that connect via IPv6
    #[arg(long, action = SetFalse)]
    pub no_ipv6: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Protocol {
    Https,
    Http,
    Rsync,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Https => write!(f, "https"),
            Self::Http => write!(f, "http"),
            Self::Rsync => write!(f, "rsync"),
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Sort {
    Age,
    Score,
}
