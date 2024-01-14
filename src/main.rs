//! `rorrim` generates an up to date mirrorlist suitable for use with `pacman` on Arch Linux.

mod mirrorlist;
mod opts;

use crate::mirrorlist::MirrorList;
use clap::Parser;
use opts::{Args, Protocol};
use std::{fs, io};

/// Default amount of mirrors to output.
const MIRROR_NUM: usize = 5;
/// Default URL to fetch the unfiltered list of mirrors.
const MIRROR_URL: &str = "https://archlinux.org/mirrors/status/json/";

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let num = args.number.unwrap_or(MIRROR_NUM);
    let url = args.url.as_deref().unwrap_or(MIRROR_URL);

    let mut list = MirrorList::get(url)?;

    // filter country
    if let Some(countries) = args.country {
        list.filter(|mirror| countries.eq_ignore_ascii_case(mirror.country.as_str()));
    }

    // filter protocols
    if let Some(protocols) = args.protocol {
        list.filter(|mirror| {
            protocols
                .iter()
                .any(|proto| mirror.protocol.parse::<Protocol>().unwrap() == *proto)
        });
    }

    // filter misc
    list.filter(|mirror| {
        let m = (mirror.isos, mirror.ipv4, mirror.ipv6);
        let a = (args.isos, args.ipv4, args.ipv6);
        m == a
    });

    // sort after filtering
    if let Some(sorter) = args.sort {
        list.sort(&sorter);
    }

    match args.save {
        Some(ref path) => list.save(num, fs::File::create(path)?),
        None => list.save(num, io::stdout()),
    }?;

    Ok(())
}
