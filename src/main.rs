use crate::mirrors::Mirror;
use crate::mirrors::MirrorList;

use chrono::prelude::*;
use clap::{ArgAction::*, Parser, ValueEnum, ValueHint};
use serde::Deserialize;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

mod mirrors;

const MIRROR_NUM: usize = 10;
const MIRROR_URL: &str = "https://archlinux.org/mirrors/status/json/";

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    /// Custom URL used to fetch all mirrors
    #[arg(short, long, value_name = "URL", value_hint = ValueHint::Url)]
    url: Option<String>,

    /// Location to save mirrorlist to
    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    save: Option<PathBuf>,

    /// Number of mirrors to output to file
    #[arg(short, long, value_name = "NUM")]
    number: Option<usize>,

    /// Protocols allowed to use
    #[arg(
        short,
        long,
        value_name = "http|https|ftp",
        value_enum,
        required = true
    )]
    protocol: Vec<Protocol>,

    /// Restrict mirrors to a set of countries or just one
    #[arg(short, long, value_name = "France", action = Append, required = true)]
    country: Vec<String>,

    /// Sort filtered mirrors by their mirror "score" or time since last sync
    #[arg(long, value_name = "age|score", value_enum)]
    sort: Option<String>,

    /// Do not use mirrors that serve ISOs
    #[arg(long, action = SetFalse)]
    no_iso: bool,

    /// Do not use mirrors that use IPv4
    #[arg(long, action = SetFalse)]
    no_ipv4: bool,

    /// Do not use mirrors that use IPv6
    #[arg(long, action = SetFalse)]
    no_ipv6: bool,

    /// Print additional information
    #[arg(long, action = SetTrue)]
    verbose: Option<bool>,
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Protocol {
    Https,
    Http,
    Rsync,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Https => write!(f, "https"),
            Self::Http => write!(f, "http"),
            Self::Rsync => write!(f, "rsync"),
        }
    }
}

#[derive(Debug, ValueEnum, PartialEq, Eq, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Sort {
    Age,
    Score,
}

fn write_file(path: Option<PathBuf>, mirrors: Vec<&Mirror>) -> std::io::Result<()> {
    let info = "# generated by rorrim";
    let date = Utc::now().format("# %C-%b-%Y at %H:%M:%S UTC");
    let output = format!("{info}\n{date}\n\n{}\n", {
        mirrors
            .iter()
            .map(|mirror| mirror.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    });

    match path {
        Some(p) => fs::File::create(p)?.write_all(output.as_bytes())?,
        None => print!("{output}"),
    };

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let req: MirrorList = mirrors::get_mirrors(&args.url.unwrap_or(MIRROR_URL.to_string()))?;

    let mirrors: Vec<&Mirror> = req
        .urls
        .iter()
        .filter(|m| args.country.contains(&m.country))
        .filter(|m| {
            args.protocol
                .iter()
                .map(|proto| proto.to_string())
                .any(|proto| proto == m.protocol)
        })
        .filter(|_| args.no_iso && args.no_ipv4 && args.no_ipv6)
        .take(args.number.unwrap_or(MIRROR_NUM))
        .collect();

    write_file(args.save, mirrors)?;
    Ok(())
}
