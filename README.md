[![ci](https://github.com/kbzt/rorrim/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/kbzt/rorrim/actions/workflows/ci.yml) [![license](https://img.shields.io/github/license/kbzt/kleidi.ca)](https://opensource.org/licenses/MIT)

# rorrim

Fetch, filter, sort, and output an up-to-date mirrorlist for Arch Linux

## Todo

- Create PKGBUILD and publish to AUR
- Implement sorting by last_sync and download rate
- Add version tagging and releases to actions
- Caching JSON response for an arbitrary TTL

## Usage

`rorrim` only requires a country and protocol to run. All other arguments are optional but recommended for better performance. A basic usage can be:

```sh
rorrim --country Canada --protocol https --number 5 --sort score --save /etc/pacman.d/mirrorlist
```

The above will output 5 mirrors in Canada that use https to retrieve data, sort them by their [score](https://archlinux.org/mirrors/status/) and place the output in `/etc/pacman.d/mirrorlist`. Note that this will path requires `rorrim` to be run as root or have the adequate permission for writing files.

<br>

To use multiple countries and/or protocols, call the argument each time:

```sh
rorrim --country Canada --country Sweden --protocol https --protocol rsync
```

Unless a path is provided, mirrors will be output to standard output. Enabling verbose logging will print additional information to stderr.

## Contributing

Contributions of any kind are more than welcome, feel free to open an issue for a feature you would like to see implemented or a pull request if you had something in mind and I will try to respond as soon as I can.

<br>

The only guideline to follow is running `cargo fmt` before commits.
## Building

To get started, clone the repo and use:

```sh
cargo run
```

To build the release version:

```sh
cargo build --release
```

## License

This software is licensed under the MIT License.
