[![ci](https://github.com/kbujari/rorrim/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/kbujari/rorrim/actions/workflows/ci.yml) [![license](https://img.shields.io/github/license/kbujari/kleidi.ca)](https://opensource.org/licenses/MIT)

# rorrim

Fetch, filter, sort, and output an up-to-date mirrorlist for Arch Linux. Aiming for feature parity with [`reflector`](https://wiki.archlinux.org/title/Reflector) initially, with extra functionality built-in.

## Usage

`rorrim` doesn't require any arguments but it's recommended to setup filters in your region for the best performance. For example:

```sh
rorrim --country Canada --protocol https --number 5 --sort score --save /etc/pacman.d/mirrorlist
```

The above will output 5 mirrors in Canada that use https to retrieve data, sort them by their [score](https://archlinux.org/mirrors/status/) and place the output in `/etc/pacman.d/mirrorlist`.

<br>

To use multiple countries and/or protocols:

```sh
rorrim -c Canada -c Sweden -p https -p rsync
```

### Details

- Writes to stdout unless file is provided
- Out of sync mirrors are discarded before any other filters are applied

## Contributing

Contributions of any kind are more than welcome, feel free to open an issue for a feature you would like to see implemented or a pull request if you had something in mind and I will try to respond as soon as I can.

<br>

The only guideline to properly format and lint your code with `rustfmt` and `clippy` before commits.

## TODO

- Implement sorting by HTTP download speed
- Add version tagging and releases to actions
- Caching initial response for faster back to back runs
- Allow user to specify custom filters?
- Work as both library and executable

## License

This software is licensed under the MIT License.
