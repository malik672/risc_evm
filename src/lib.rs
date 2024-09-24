pub mod compiler;

use core::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

use alloy_primitives::{ruint::ToUintError, U256};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct MyU256(U256);

impl TryFrom<[u8; 32]> for MyU256 {
    type Error = ToUintError<U256>; // Define an appropriate error type

    fn try_from(value: [u8; 32]) -> Result<Self, Self::Error> {
        let mut result = U256::default();
        for (i, &byte) in value.iter().enumerate() {
            result |= U256::from(byte) << (8 * (31 - i));
        }
        Ok(MyU256(result))
    }
}

// Addition
impl Add for MyU256 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        MyU256(self.0.overflowing_add(other.0).0)
    }
}

// Subtraction
impl Sub for MyU256 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        MyU256(self.0.overflowing_sub(other.0).0)
    }
}

// Multiplication
impl Mul for MyU256 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        MyU256(self.0.overflowing_mul(other.0).0)
    }
}

// Division
impl Div for MyU256 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        MyU256(self.0.checked_div(other.0).unwrap_or(U256::ZERO))
    }
}


// Display implementation for easy printing
impl fmt::Display for MyU256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Rem for MyU256 {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        if other.0.is_zero() {
            MyU256(U256::ZERO)
        } else {
            MyU256(self.0 % other.0)
        }
    }
}


impl MyU256 {
    pub const MAX: U256 = U256::MAX;
}