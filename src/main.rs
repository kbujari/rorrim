mod mirrorlist;
mod opts;

use clap::Parser;
use std::{fs, io};

use crate::mirrorlist::MirrorList;

const MIRROR_NUM: usize = 10;
const MIRROR_URL: &str = "https://archlinux.org/mirrors/status/json/";

fn main() -> anyhow::Result<()> {
    let args = opts::Args::parse();

    let num = args.number.unwrap_or(MIRROR_NUM);
    let url = args.url.unwrap_or(MIRROR_URL.to_string());

    let mut req = MirrorList::get(&url)?;

    if let Some(countries) = args.country {
        req.urls.retain(|v| countries.contains(&v.country));
    }

    if let Some(protocols) = args.protocol {
        req.urls.retain(|v| {
            protocols
                .iter()
                .any(|proto| proto.to_string() == v.protocol)
        });
    }

    req.urls
        .retain(|_| args.no_iso && args.no_ipv4 && args.no_ipv6);

    if let Some(sorter) = args.sort {
        req.sort(&sorter);
    }

    let mut writer: Box<dyn io::Write> = if let Some(path) = args.save {
        Box::new(fs::File::create(path)?)
    } else {
        Box::new(io::stdout())
    };

    req.save(num, &mut writer)?;

    Ok(())
}
