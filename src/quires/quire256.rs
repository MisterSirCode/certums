#![allow(dead_code)]

use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// A Quire-like object (Relevant to Posit math operations). 
/// Essentially a custom signed integer that works up to 256 bits.
/// 
/// Used by different types of 128-bit Certums to multiply without losing precision
pub struct u256 {
    bits: (u128, u128)
}

impl Add for u256 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let over_lower = u128::overflowing_add(self.bits.0, rhs.bits.0);
        let partial_lower;
        let carry;
        if over_lower.1 {
            
        }
        let sum_upper = u128::saturating_add(u128::saturating_add(self.bits.1, rhs.bits.1), carry);
        let sum_lower = u128::saturating_add(self, rhs)

    }
}

impl u256 {

}