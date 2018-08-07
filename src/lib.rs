// Copyright 2016 Ordinal Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate num_integer;

use std::fmt;

use num_integer::Integer;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ordinal<T: Integer + fmt::Display>(T);

impl<T> fmt::Display for Ordinal<T>
where
    T: Integer + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let s = format!("{}", self.0);
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

impl<T> From<T> for Ordinal<T>
where
    T: Integer + fmt::Display,
{
    fn from(t: T) -> Self {
        Ordinal(t)
    }
}

pub trait ToOrdinal<T>
where
    T: Integer + fmt::Display,
{
    fn ordinal(self) -> Ordinal<T>;
}

impl<T> ToOrdinal<T> for T
where
    T: Integer + fmt::Display,
{
    fn ordinal(self) -> Ordinal<T> {
        Ordinal(self)
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
            assert_eq!(case.1, format!("{}", Ordinal::from(case.0)));
        }
    }

    #[test]
    fn test_types() {
        assert_eq!("1st", format!("{}", 1i8.ordinal()));
        assert_eq!("1st", format!("{}", 1i16.ordinal()));
        assert_eq!("1st", format!("{}", 1i32.ordinal()));
        assert_eq!("1st", format!("{}", 1i64.ordinal()));
        assert_eq!("1st", format!("{}", 1isize.ordinal()));

        assert_eq!("1st", format!("{}", 1u8.ordinal()));
        assert_eq!("1st", format!("{}", 1u16.ordinal()));
        assert_eq!("1st", format!("{}", 1u32.ordinal()));
        assert_eq!("1st", format!("{}", 1u64.ordinal()));
        assert_eq!("1st", format!("{}", 1usize.ordinal()));

        assert_eq!("1st", format!("{}", BigInt::one().ordinal()));
        assert_eq!("1st", format!("{}", BigUint::one().ordinal()));
    }
}
