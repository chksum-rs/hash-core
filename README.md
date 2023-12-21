# chksum-hash-core

[![crates.io](https://img.shields.io/crates/v/chksum-hash-core?style=flat-square&logo=rust "crates.io")](https://crates.io/crates/chksum-hash-core)
[![Build](https://img.shields.io/github/actions/workflow/status/chksum-rs/hash-core/rust.yml?branch=master&style=flat-square&logo=github "Build")](https://github.com/chksum-rs/hash-core/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/chksum-hash-core?style=flat-square&logo=docsdotrs "docs.rs")](https://docs.rs/chksum-hash-core/)
[![MSRV](https://img.shields.io/badge/MSRV-1.58.0-informational?style=flat-square "MSRV")](https://github.com/chksum-rs/hash-core/blob/master/Cargo.toml)
[![deps.rs](https://deps.rs/crate/chksum-hash-core/0.0.0/status.svg?style=flat-square "deps.rs")](https://deps.rs/crate/chksum-hash-core/0.0.0)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square "unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![LICENSE](https://img.shields.io/github/license/chksum-rs/hash-core?style=flat-square "LICENSE")](https://github.com/chksum-rs/hash-core/blob/master/LICENSE)

Core traits and functions for batch and stream hash computation.

## Setup

To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:

```toml
[dependencies]
chksum-hash-core = "0.0.0"
```

Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:

```sh
cargo add chksum-hash-core
```

## Example Crates

For implementation-specific examples, refer to the source code of the following crates:

* [`chksum-hash-md5`](https://github.com/chksum-rs/hash-md5)
* [`chksum-hash-sha1`](https://github.com/chksum-rs/hash-sha1)
* [`chksum-hash-sha2`](https://github.com/chksum-rs/hash-sha2)
    * [`chksum-hash-sha2-224`](https://github.com/chksum-rs/hash-sha2-224)
    * [`chksum-hash-sha2-256`](https://github.com/chksum-rs/hash-sha2-256)
    * [`chksum-hash-sha2-384`](https://github.com/chksum-rs/hash-sha2-384)
    * [`chksum-hash-sha2-512`](https://github.com/chksum-rs/hash-sha2-512)

## License

This crate is licensed under the MIT License.
