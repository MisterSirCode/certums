#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Shl, Shr};

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// A Quire-like object (Relevant to Posit math operations). 
/// Essentially a custom signed integer that works up to 256 bits.
/// 
/// Used by different types of 128-bit Certums to multiply without losing precision
pub struct u256 {
    pub bits: (u128, u128)
}

impl Add for u256 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (ov_lo, carry_lo) = u128::overflowing_add(self.bits.0, rhs.bits.0);
        let (ov_hi, carry_hi) = u128::overflowing_add(self.bits.1, rhs.bits.1);
        let (ov_carry, carry) = u128::overflowing_add(ov_hi, carry_lo as u128);
        if carry_hi || carry {
            Self::MAX
        } else {
            Self { bits: (ov_lo, ov_carry) }
        }
    }
}

impl Shl<u128> for u256 {
    type Output = Self;
    fn shl(self, rhs: u128) -> Self {
        assert!(rhs <= 255u128);
        let (res_lo, res_hi);
        if rhs >= 128 {
            let adj = rhs - 128;
            res_lo = self.bits.1 << adj;
            res_hi = 0u128;
        } else {
            let adj = 128 - rhs;
            let ov_hi = self.bits.1 >> adj;
            res_lo = (self.bits.0 << rhs) | ov_hi;
            res_hi = self.bits.1 << rhs;
        }
        Self { bits: (res_lo, res_hi) }
    }
}

impl Shr<u128> for u256 {
    type Output = Self;
    fn shr(self, rhs: u128) -> Self {
        assert!(rhs <= 255u128);
        let (res_lo, res_hi);
        if rhs >= 128 {
            let adj = rhs - 128;
            res_hi = self.bits.0 >> adj;
            res_lo = 0u128;
        } else {
            let adj = 128 - rhs;
            let ov_lo = self.bits.0 << adj;
            res_hi = (self.bits.1 >> rhs) | ov_lo;
            res_lo = self.bits.0 >> rhs;
        }
        Self { bits: (res_lo, res_hi) }
    }
}

impl From<u256> for u128 {
    /// Convert a u256 to a u128. 
    /// 
    /// Equivalent to using `as` to cast from a bigger int to a smaller int
    fn from(value: u256) -> Self {
        value.bits.1
    }
}

impl From<u128> for u256 {
    /// Convert a u128 to a u256. 
    /// 
    /// Equivalent to using `as` to cast from a smaller int to a bigger int
    fn from(value: u128) -> Self {
        Self {
            bits: (0u128, value)
        }
    }
}

impl u256 {
    pub const MAX: u256 = Self { bits: (u128::MAX, u128::MAX) };

    pub fn saturating_mul(self, other: Self) -> Self {
        Self::MAX
    }
}