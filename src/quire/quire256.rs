#![allow(dead_code)]

use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use std::cmp::{Eq, PartialEq, Ordering};

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// A Quire-like object (Relevant to Posit math operations). 
/// Essentially a custom unsigned integer that works up to 256 bits.
/// 
/// Used by different types of 128-bit Certums to multiply without losing precision
pub struct u256 {
    pub bits: [u128; 2]
}

impl Ord for u256 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bits.cmp(&other.bits)
    }
}

impl Eq for u256 { }

impl PartialOrd for u256 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for u256 {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }

    fn ne(&self, other: &Self) -> bool {
        self.bits != other.bits
    }
}

impl PartialOrd<u128> for u256 {
    fn partial_cmp(&self, other: &u128) -> Option<Ordering> {
        Some(self.cmp(&u256::from(*other)))
    }
}

impl PartialEq<u128> for u256 {
    fn eq(&self, other: &u128) -> bool {
        self.bits == u256::from(*other).bits
    }

    fn ne(&self, other: &u128) -> bool {
        self.bits != u256::from(*other).bits
    }
}

impl Add for u256 {
    type Output = Self;
    /// Add two u256s together. Behaves like saturating_add
    fn add(self, rhs: Self) -> Self {
        let (ov_right, c1) = u128::overflowing_add(self.bits[1], rhs.bits[1]);
        let (ov_left, c2) = u128::overflowing_add(self.bits[0], rhs.bits[0]);
        let (ov_carry, c3) = u128::overflowing_add(ov_left, c1 as u128);
        if c2 || c3 {
            Self::MAX
        } else {
            Self { bits: [ov_carry, ov_right] }
        }
    }
}

impl AddAssign for u256 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Add<u128> for u256 {
    type Output = Self;
    /// Add a u128 to a u256. Behaves like saturating_add
    fn add(self, rhs: u128) -> Self {
        self + Self::from(rhs)
    }
}

impl AddAssign<u128> for u256 {
    fn add_assign(&mut self, rhs: u128) {
        *self = *self + rhs
    }
}

impl Sub for u256 {
    type Output = Self;
    /// Subtract two u256s. Behaves like saturating_sub
    fn sub(self, rhs: Self) -> Self {
        if rhs >= self { return Self::MIN }
        let (ov_right, c1) = u128::overflowing_sub(self.bits[1], rhs.bits[1]);
        let (ov_left, c2) = u128::overflowing_sub(self.bits[0], rhs.bits[0]);
        let (ov_carry, c3) = u128::overflowing_sub(ov_left, c1 as u128);
        if c2 || c3 {
            Self::MIN
        } else {
            Self { bits: [ov_carry, ov_right] }
        }
    }
}

impl SubAssign for u256 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Sub<u128> for u256 {
    type Output = Self;
    /// Subtract a u128 from a u256. Behaves like saturating_sub
    fn sub(self, rhs: u128) -> Self {
        self - Self::from(rhs)
    }
}

impl SubAssign<u128> for u256 {
    fn sub_assign(&mut self, rhs: u128) {
        *self = *self - rhs
    }
}

impl Mul for u256 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self { bits: [0, 0] }
    }
}

impl MulAssign for u256 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Shl<u128> for u256 {
    type Output = Self;
    fn shl(self, rhs: u128) -> Self {
        assert!(rhs <= 255u128);
        let (res_left, res_right);
        if rhs >= 128 {
            let adj = rhs - 128;
            res_left = self.bits[1] << adj;
            res_right = 0u128;
        } else {
            let adj = 128 - rhs;
            let ov_right = self.bits[1] >> adj;
            res_left = (self.bits[0] << rhs) | ov_right;
            res_right = self.bits[1] << rhs;
        }
        Self { bits: [res_left, res_right] }
    }
}

impl ShlAssign<u128> for u256 {
    fn shl_assign(&mut self, rhs: u128) {
        *self = *self << rhs;
    }
}

impl Shr<u128> for u256 {
    type Output = Self;
    fn shr(self, rhs: u128) -> Self {
        assert!(rhs <= 255u128);
        let (res_left, res_right);
        if rhs >= 128 {
            let adj = rhs - 128;
            res_right = self.bits[0] >> adj;
            res_left = 0u128;
        } else {
            let adj = 128 - rhs;
            let ov_left = self.bits[0] << adj;
            res_right = (self.bits[1] >> rhs) | ov_left;
            res_left = self.bits[0] >> rhs;
        }
        Self { bits: [res_left, res_right] }
    }
}

impl ShrAssign<u128> for u256 {
    fn shr_assign(&mut self, rhs: u128) {
        *self = *self >> rhs;
    }
}

impl BitAnd for u256 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self { bits: [self.bits[0] & rhs.bits[0], self.bits[1] & rhs.bits[1]] }
    }
}

impl BitAndAssign for u256 {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for u256 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self { bits: [self.bits[0] | rhs.bits[0], self.bits[1] | rhs.bits[1]] }
    }
}

impl BitOrAssign for u256 {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for u256 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self { bits: [self.bits[0] ^ rhs.bits[0], self.bits[1] ^ rhs.bits[1]] }
    }
}

impl BitXorAssign for u256 {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl From<u256> for u128 {
    /// Convert a u256 to a u128. 
    /// 
    /// Equivalent to using `as` to cast from a bigger int to a smaller int
    fn from(value: u256) -> Self {
        value.bits[1]
    }
}

impl From<u128> for u256 {
    /// Convert a u128 to a u256. 
    /// 
    /// Equivalent to using `as` to cast from a smaller int to a bigger int
    fn from(value: u128) -> Self {
        Self {
            bits: [0u128, value]
        }
    }
}

impl u256 {
    /// Maximum u256 Integer
    /// 
    /// Decimal Equivalent: 115792089237316195423570985008687907852589419931798687112530834793049593217025
    pub const MAX: u256 = Self { bits: [u128::MAX, u128::MAX] };
    /// Minimum u256 Interger
    /// 
    /// Decimal Equivalent: 0
    pub const MIN: u256 = Self { bits: [0, 0] };

    pub fn raw(arr: [u128; 2]) -> Self {
        Self { bits: arr }
    }

    /// Multiply two u128's and return a u256
    pub fn mul_u128s(lhs: u128, rhs: u128) -> Self {
        Self::MAX
    }

    /// Multiply two u256's and clamp within the MIN and MAX numeric range
    pub fn saturating_mul(self, other: Self) -> Self {
        Self::MAX
    }
}