# Ordinals (1st, 2nd, 3rd etc) for Rust

[![test](https://github.com/gleich/ordinal/actions/workflows/test.yml/badge.svg)](https://github.com/gleich/ordinal/actions/workflows/test.yml)
[![lint](https://github.com/gleich/ordinal/actions/workflows/lint.yml/badge.svg)](https://github.com/gleich/ordinal/actions/workflows/lint.yml)

This crate provides a type `Ordinal<T>` that formats an [`Integer`](https://docs.rs/num-integer/0.1/num_integer/trait.Integer.html) type `T` as an [ordinal number](https://en.wikipedia.org/wiki/Ordinal_number_%28linguistics%29) (1st, 2nd, 3rd, etc).

## Install

Just add the following line to your `Cargo.toml` file:

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

## Ownership Context

I am not the original author of this crate. The original author, [@dtolnay](https://github.com/dtolnay), didn't want to maintain the crate anymore so he gave it to me. This means that I am the official maintainer of this package. If you want to see where this decision was made check the [GitHub issue](https://github.com/gleich/ordinal/issues/2).
