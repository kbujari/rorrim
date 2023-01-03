use std::fs;

mod request;
mod cli;

use self::request::Request;
use self::cli::{MIRROR_NUM,MIRROR_URL};

fn main() {
  // let req = reqwest::blocking::get(MIRROR_URL).unwrap();

  let args = <cli::Args as clap::Parser>::parse();
  let req = fs::read_to_string("sample.json").unwrap();
  let var: Request = serde_json::from_str(&req).unwrap();
  // let var: Request = req.json().unwrap();
  let can: Vec<&request::Mirror> = var.urls.iter().filter(|x| {
      args.countries.contains(&x.country)
  }).filter(|x| !x.ipv4)
  .take(MIRROR_NUM.into()).collect();

  println!("Taken from {}", MIRROR_URL);
  for mirror in can {
      println!("{} - {}", mirror.url, mirror.country_code);
  }
}
