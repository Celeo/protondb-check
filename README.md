# protondb_search

[![Crates.io](https://img.shields.io/crates/v/protondb_search)](https://crates.io/crates/protondb_search)
[![License](https://img.shields.io/crates/l/protondb_search)](LICENSE)

A small CLI utility to get game inforamtion from [ProtonDB](https://www.protondb.com/).

This program is not affiliated with **ProtonDB**, **Algolia**, or **Steam** in any way.

## Building

1. Install [Rust](https://rust-lang.org/)
1. Clone the repo
1. Run `cargo build`

## Installing

You can build as above and then run `cargo install --path .`, or install from [crates.io](https://crates.io/) with `cargo install protondb_search`.

## Usage

Run the utility with the name of a game:

```sh
$ protondb_search "eve online"
$ protondb_search "monster hunter"
$ protondb_search "runescape"
```

## License

MIT
