#![feature(integer_sign_cast)]

pub mod compiler;

use core::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

use alloy_primitives::{ruint::ToUintError, U256};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct MyU256(U256);

impl TryFrom<[u8; 32]> for MyU256 {
    type Error = ToUintError<U256>; 

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

    fn u256_to_usize(bytes: [u8; 32]) -> usize {
        let mut result = 0usize;
        for &byte in bytes.iter().rev().take(8) {
            result = (result << 8) | (byte as usize);
        }
        result
    }

    pub fn to_be_bytes(&self) -> [u8; 32] {
        self.0.to_be_bytes()
    }

    pub fn from_be_bytes(bytes: [u8; 32]) -> Self {
        MyU256(U256::from_be_bytes(bytes))
    }

    pub fn as_usize(&self) -> usize {
        let bytes = self.0.to_be_bytes();
        Self::u256_to_usize(bytes)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct I256(U256);

impl I256 {
    pub fn from_be_bytes(bytes: [u8; 32]) -> Self {
        I256(U256::from_be_bytes(bytes))
    }

    pub fn to_be_bytes(&self) -> [u8; 32] {
        self.0.to_be_bytes()
    }

    pub fn abs(&self) -> Self {
        if self.is_negative() {
            I256((!self.0).overflowing_add(U256::from(1)).0)
        } else {
            *self
        }
    }

    pub fn is_negative(&self) -> bool {
        self.0.bit(255)
    }
}

impl Rem for I256 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        // This is a simplified version and may not handle all edge cases(so faulty)
        let a = self.0;
        let b = rhs.0;
        let r = a % b;
        let sign = if self.is_negative() ^ rhs.is_negative() {
            -1
        } else {
            1
        };
        // I256(r * sign);
        todo!()
    }
}

impl PartialOrd for I256 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_negative() && !other.is_negative() {
            Some(std::cmp::Ordering::Less)
        } else if !self.is_negative() && other.is_negative() {
            Some(std::cmp::Ordering::Greater)
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}
