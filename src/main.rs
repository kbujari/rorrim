use std::fs;
use std::io::Write;

mod cli;
mod request;

use self::cli::{MIRROR_NUM, MIRROR_URL};
use self::request::{Mirror, Request};

fn main() {
  let args = <cli::Args as clap::Parser>::parse();

  // let req = reqwest::blocking::get(MIRROR_URL).unwrap();
  // let var: Request = req.json().unwrap();

  let req = fs::read_to_string("sample.json").unwrap();
  let var: Request = serde_json::from_str(&req).unwrap();

  let can: Vec<&Mirror> = var
    .urls
    .iter()
    .filter(|x| {
      args.country.contains(&x.country)
        && args
          .protocol
          .iter()
          .map(|x| match x {
            cli::Protocol::Https => "https",
            cli::Protocol::Http => "http",
            cli::Protocol::Rsync => "rsync",
          })
          .any(|n| n == x.protocol)
        && args.no_iso
        && args.no_ipv4
        && args.no_ipv6
    })
    .take(MIRROR_NUM.into())
    .collect();

  let mut file = fs::File::create(&args.save.expect("No file set")).expect("Unable to create file");
  can.iter().for_each(|line| {
    file
      .write_all(format!("{}\n", line.url).as_bytes())
      .unwrap()
  });

  println!("Taken from {}", MIRROR_URL);
  can.iter().for_each(|mirror| println!("{}", mirror.url));
}
