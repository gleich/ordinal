Ordinals (1st, 2nd, 3rd etc) for Rust
=====================================

[![Build Status](https://api.travis-ci.org/dtolnay/ordinal.svg?branch=master)](https://travis-ci.org/dtolnay/ordinal)
[![Latest Version](https://img.shields.io/crates/v/ordinal.svg)](https://crates.io/crates/ordinal)

This crate provides a type `Ordinal<T>` that formats an [`Integer`] type `T` as
an [ordinal number] (1st, 2nd, 3rd etc).

[`Integer`]: https://docs.rs/num-integer/0.1/num_integer/trait.Integer.html
[ordinal number]: https://en.wikipedia.org/wiki/Ordinal_number_%28linguistics%29

```toml
[dependencies]
ordinal = "0.2"
```

## Example

```rust
use ordinal::Ordinal;

fn main() {
    assert_eq!("2nd", Ordinal(2).to_string());
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
