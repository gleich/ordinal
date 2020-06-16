//! [![github]](https://github.com/dtolnay/ordinal)&ensp;[![crates-io]](https://crates.io/crates/ordinal)&ensp;[![docs-rs]](https://docs.rs/ordinal)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! This crate provides a type `Ordinal<T>` that formats an [`Integer`] type `T` as
//! an [ordinal number] (1st, 2nd, 3rd etc).
//!
//! [`Integer`]: https://docs.rs/num-integer/0.1/num_integer/trait.Integer.html
//! [ordinal number]: https://en.wikipedia.org/wiki/Ordinal_number_%28linguistics%29
//!
//! # Example
//!
//! ```
//! use ordinal::Ordinal;
//!
//! fn main() {
//!     assert_eq!("2nd", Ordinal(2).to_string());
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/ordinal/0.2.3")]

use std::fmt::{self, Display};

use num_integer::Integer;

/// Newtype wrapper struct that formats integers as an ordinal number.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ordinal<T>(pub T);

impl<T> Display for Ordinal<T>
where
    T: Integer + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.to_string();
        let suffix = if s.ends_with("1") && !s.ends_with("11") {
            "st"
        } else if s.ends_with("2") && !s.ends_with("12") {
            "nd"
        } else if s.ends_with("3") && !s.ends_with("13") {
            "rd"
        } else {
            "th"
        };
        write!(f, "{}{}", s, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::{BigInt, BigUint};
    use num_traits::One;

    #[test]
    fn test_display() {
        for case in vec![
            (-4, "-4th"),
            (-3, "-3rd"),
            (-2, "-2nd"),
            (-1, "-1st"),
            (0, "0th"),
            (1, "1st"),
            (2, "2nd"),
            (3, "3rd"),
            (4, "4th"),
            (10, "10th"),
            (11, "11th"),
            (12, "12th"),
            (13, "13th"),
            (14, "14th"),
            (20, "20th"),
            (21, "21st"),
            (22, "22nd"),
            (23, "23rd"),
            (24, "24th"),
            (100, "100th"),
            (101, "101st"),
            (102, "102nd"),
            (103, "103rd"),
            (104, "104th"),
            (110, "110th"),
            (111, "111th"),
            (112, "112th"),
            (113, "113th"),
            (114, "114th"),
            (120, "120th"),
            (121, "121st"),
            (122, "122nd"),
            (123, "123rd"),
            (124, "124th"),
        ] {
            assert_eq!(case.1, Ordinal(case.0).to_string());
        }
    }

    #[test]
    fn test_types() {
        assert_eq!("1st", Ordinal(1i8).to_string());
        assert_eq!("1st", Ordinal(1i16).to_string());
        assert_eq!("1st", Ordinal(1i32).to_string());
        assert_eq!("1st", Ordinal(1i64).to_string());
        assert_eq!("1st", Ordinal(1isize).to_string());

        assert_eq!("1st", Ordinal(1u8).to_string());
        assert_eq!("1st", Ordinal(1u16).to_string());
        assert_eq!("1st", Ordinal(1u32).to_string());
        assert_eq!("1st", Ordinal(1u64).to_string());
        assert_eq!("1st", Ordinal(1usize).to_string());

        assert_eq!("1st", Ordinal(BigInt::one()).to_string());
        assert_eq!("1st", Ordinal(BigUint::one()).to_string());
    }
}
