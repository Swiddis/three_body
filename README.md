<!-- SPDX-License-Identifier: MIT -->

# Three Body

[![License](https://img.shields.io/badge/license-MIT-informational?style=flat-square)](COPYRIGHT.md)
[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

Yet another three-body simulation in Rust.
Built with:

- [Kiss3d](https://crates.io/crates/kiss3d) for graphics
- [Rapier](https://rapier.rs/) for physics

A neat unique thing here, compared to other three-body simulations, is that collisions are handled elastically.

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Install

Install [Rust](https://www.rust-lang.org/), which includes Cargo.

```bash
cargo install
```

## Usage

```bash
cargo run --release
```

## Contributing

Contributions welcome, just make a PR!

## License

&copy; 2022 Simeon Widdis.

This project is licensed under the [MIT license](https://opensource.org/licenses/MIT) ([`LICENSE`](LICENSE)).
