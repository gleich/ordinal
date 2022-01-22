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
#![allow(clippy::unseparated_literal_suffix)]

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
        write!(f, "{}{}", self.0, self.suffix())
    }
}

impl<T> Ordinal<T>
where
    T: Integer + Display,
{
    /// Returns just the suffix for the ordinal
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("nd", Ordinal(2).suffix());
    /// ```
    pub fn suffix(&self) -> &str {
        let s = self.0.to_string();
        if s.ends_with('1') && !s.ends_with("11") {
            "st"
        } else if s.ends_with('2') && !s.ends_with("12") {
            "nd"
        } else if s.ends_with('3') && !s.ends_with("13") {
            "rd"
        } else {
            "th"
        }
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

    #[test]
    fn test_suffix() {
        for case in vec![
            (-4, "th"),
            (-3, "rd"),
            (-2, "nd"),
            (-1, "st"),
            (0, "th"),
            (1, "st"),
            (2, "nd"),
            (3, "rd"),
            (4, "th"),
            (10, "th"),
            (11, "th"),
            (12, "th"),
            (13, "th"),
            (14, "th"),
            (20, "th"),
            (21, "st"),
            (22, "nd"),
            (23, "rd"),
            (24, "th"),
            (100, "th"),
            (101, "st"),
            (102, "nd"),
            (103, "rd"),
            (104, "th"),
            (110, "th"),
            (111, "th"),
            (112, "th"),
            (113, "th"),
            (114, "th"),
            (120, "th"),
            (121, "st"),
            (122, "nd"),
            (123, "rd"),
            (124, "th"),
        ] {
            assert_eq!(case.1, Ordinal(case.0).suffix());
        }
    }
}
