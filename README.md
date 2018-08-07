Ordinals (1st, 2nd, 3rd etc) for Rust
=====================================

[![Build Status](https://api.travis-ci.org/dtolnay/ordinal.svg?branch=master)](https://travis-ci.org/dtolnay/ordinal)
[![Latest Version](https://img.shields.io/crates/v/ordinal.svg)](https://crates.io/crates/ordinal)

This crate provides a type `Ordinal<T>` that formats an [`Integer`] type `T` as
an [ordinal number] (1st, 2nd, 3rd etc).

[`Integer`]: https://docs.rs/num-integer/0.1/num_integer/trait.Integer.html
[ordinal number]: https://en.wikipedia.org/wiki/Ordinal_number_%28linguistics%29


## Installation

Ordinal is available on [crates.io](https://crates.io/crates/ordinal). Use the
following in `Cargo.toml`:

```toml
[dependencies]
ordinal = "0.2"
```

## Using Ordinal

```rust
extern crate ordinal;
use ordinal::Ordinal;

fn main() {
    assert_eq!("2nd", Ordinal(2).to_string());
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
