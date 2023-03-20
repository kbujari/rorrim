use crate::mirrors::Mirror;
use crate::mirrors::MirrorList;

use chrono::prelude::*;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

mod cli;
mod mirrors;

const MIRROR_NUM: usize = 10;
const MIRROR_URL: &'static str = "https://archlinux.org/mirrors/status/json/";

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
  let args = <cli::Args as clap::Parser>::parse();

  let req: MirrorList = mirrors::get_mirrors(&args.url.unwrap_or(MIRROR_URL.to_string()))?;

  let mirrors: Vec<&Mirror> = req
    .urls
    .iter()
    .filter(|m| args.country.contains(&m.country))
    .filter(|m| {
      args
        .protocol
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
