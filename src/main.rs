mod request;

use std::{fs, path::PathBuf};
use serde::Deserialize;
use clap::Parser;

#[derive(Debug, Deserialize)]
struct Request {
    cutoff: u32,
    last_check: String,
    num_checks: u8,
    check_frequency: u16,
    urls: Vec<Mirror>,
    version: u8,
}

#[derive(Debug, Deserialize)]
struct Mirror {
    url: String,
    protocol: String,
    last_sync: Option<String>,
    completion_pct: f32,
    delay: Option<i32>,
    duration_avg: Option<f32>,
    duration_stddev: Option<f32>,
    score: Option<f64>,
    active: bool,
    country: String,
    country_code: String,
    isos: bool,
    ipv4: bool,
    ipv6: bool,
    details: String
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
struct Args {
    /// Custom URL used to fetch all mirrors [default]
    url: Option<String>,

    /// Location to save mirrorlist to
    #[arg(short, long, value_name = "file")]
    save: Option<PathBuf>,

    /// Log actions to stdout
    #[arg(short, long)]
    verbose: Option<bool>,

    /// Number of mirrors to output to file
    #[arg(short, long, value_name = "num")]
    number: Option<u16>,

    /// Parameter to sort the mirrorlist by
    #[arg(long)]
    sort: String
}

fn main() {
    //let req = reqwest::blocking::get("https://archlinux.org/mirrors/status/json/").unwrap();

    let args = Args::parse();
    //let req = fs::read_to_string("sample.json").unwrap();
    //let var: Request = serde_json::from_str(&req).unwrap(); // req.json().unwrap();
    //dbg!(&var.urls[0]);
}
