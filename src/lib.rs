// Copyright 2016 Ordinal Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_root_url = "https://docs.rs/ordinal/0.2.0")]

extern crate num_integer;

use std::fmt::{self, Display};

use num_integer::Integer;

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
    extern crate num_bigint;
    extern crate num_traits;

    use self::num_bigint::{BigInt, BigUint};
    use self::num_traits::One;
    use super::*;

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
