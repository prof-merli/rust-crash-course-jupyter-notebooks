//! # A little example lib for math functions
//!
//! Helper functions to support arithmetic operations
//!
//! ...
//!
//! ## Code Example
//!
//! ```
//! # use example_math_tools::{addi, subi, multi};
//! let x = addi(4, 6).0;
//! let y = subi(4, 6).0;
//! let z = multi(3, 4).0;
//! let overflow = addi(-3, -4).1;
//! let (w, overflow_w) = subi(12, 34);
//! # assert_eq!(x, 10);
//! # assert_eq!(y, -2);
//! # assert_eq!(z, 12);
//! # assert_eq!(overflow, false);
//! # assert_eq!(w, -22);
//! # assert_eq!(overflow_w, false);
//! ```

use anyhow::{bail, Result};
use std::io;

/// Adds up two 64bit signed integers.
///
/// When the addition overflows, the bool of the return value will be true.
///
/// # Arguments
///
/// * `x` - first integer
/// * `y` - second integer
///
/// # Return value
///
/// `(i64, bool)` - Tuple with the result of the addition and a flag that states whether and overflow happened or not.
pub fn addi(x: i64, y: i64) -> (i64, bool) 
{
    x.overflowing_add(y)
}

pub fn subi(x: i64, y: i64) -> (i64, bool) 
{
    x.overflowing_sub(y)
}

pub fn multi(x: i64, y: i64) -> (i64, bool) 
{
    x.overflowing_mul(y)
}

pub fn addi_res(x: i64, y: i64) -> Result<i64> {
    let (res, overflow) = x.overflowing_add(y);

    if overflow 
    {
        bail!("ERROR: Overflow!")
    } else {
        Ok((res))
    }
}

pub fn subi_res(x: i64, y: i64) -> Result<i64> {
    let (res, overflow) = x.overflowing_sub(y);

    if overflow 
    {
        bail!("ERROR: Overflow!")
    } else {
        Ok(res)
    }
}

pub fn multi_res(x: i64, y: i64) -> Result<i64> {
    let (res, overflow) = x.overflowing_mul(y);

    if overflow 
    {
        bail!("ERROR: Overflow!")
    } else {
        Ok(res)
    }
}

pub fn square(x: i64) -> i64 {
    let (y, over) = multi(x, x);
    y
}

pub fn squareVec(x: &Vec<i64>) -> Vec<i64> {
    let mut vec1 = Vec::with_capacity(x.len());
    vec1.resize(x.len(), 0);

    for (index, e) in x.iter().enumerate() {
        vec1[index] = square(*e);
    }

    vec1
}

/// Test module
///
/// Submodule to contain the unit tests for this module.
/// This comment doesn't get printed because the module is configured as tests.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_works() {
        assert_eq!(addi(4, 6).0, 10);
        assert_eq!(addi(-4, -6).0, -10);
        assert_ne!(addi(3, 4).0, 8);
        assert_ne!(addi(-3, -4).0, -8);
    }

    #[test]
    fn add_overflow_and_underflow() {
        assert_eq!(addi(std::i64::MAX, 1).1, true);
        assert_eq!(addi(std::i64::MAX, 1).0, std::i64::MIN);
        assert_eq!(addi(std::i64::MIN, -1).1, true);
        assert_eq!(addi(std::i64::MIN, -1).0, std::i64::MAX);
    }

    #[test]
    fn square_vec_works() {
        let vec = vec![1, 2, 3, 4, 5];

        assert_eq!(squareVec(&vec), vec![1, 4, 9, 16, 25]);
    }
}
