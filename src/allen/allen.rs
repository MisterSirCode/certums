#![allow(dead_code)]

use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use std::cmp::{Eq, PartialEq, Ordering};

#[derive(Clone, Debug)]
pub struct BigInt(Vec<u8>);

impl<'a> Add<&'a Self> for BigInt {
    type Output = Self;
    fn add(mut self, rhs: &'a Self) -> Self {
        let mut carry = false;
        for i in 0..self.0.len() {
            (self.0[i], carry) = self.0[i].carrying_add(rhs.0[i], carry);
        }
        if carry {
            self.0.push(1);
        }
        self
    }
}

impl<'a> Mul<&'a Self> for BigInt {
    type Output = Self;
    fn mul(mut self, rhs: &'a Self) -> Self {
        let mut carries = vec![0];
        for (l, &r) in self.0.iter_mut().zip(&rhs.0) {
            let (res, carry) = l.carrying_mul(r, 0);
            *l = res;
            carries.push(carry);
        }
        self + &Self(carries)
    }
}

impl From<u8> for BigInt {
    fn from(value: u8) -> Self {
        BigInt(vec![value])
    }
}

#[derive(Clone, Debug)]
/// ALN - Arbitrary Length Number
/// 
/// This type is slow and only meant for precision conversions and displaying
pub struct ALN {
    sgn: bool,
    int: BigInt,
    frc: BigInt,
}

impl ALN {
    /// Create an empty ALN
    pub fn empty() -> Self {
        Self { sgn: false, int: BigInt(vec![0]), frc: BigInt(vec![0]) }
    }

    /// Rectify and Validate a string for use with ALN
    pub fn rectify_string(raw: String) -> String {
        let sgn: bool = raw.starts_with('-');
        let mut has_dp: bool = false; // Has a decimal point / Is a decimal number
        let mut cnt_int: String = Default::default();
        let mut valid: String = Default::default();
        if sgn { valid.push('-'); }
        for c in raw.chars() {
            if c.is_ascii_digit() {
                if (cnt_int.len() > 1) | (c != '0') { // Ignore leading zeroes.
                    cnt_int.push(c); // Push to integer
                    valid.push(c);
                }
            } else if (c == '.') & (!has_dp) { 
                has_dp = true; 
                valid.push('.');
            } // Reject any other character
        }
        while valid.ends_with('0') { // Eliminate trailing zeroes.
            valid.pop();
        }
        valid
    }
}

impl From<String> for ALN {
    fn from(value: String) -> Self {
        let int: String;
        let frc: String;
        let dec: bool;
        if value.contains('.') {
            let col: Vec<&str> = value.split('.').collect();
            int = col[0].to_string();
            frc = col[1].to_string();
            dec = true;
        } else {
            int = value;
            frc = Default::default();
            dec = false;
        }
        for (i, c) in int.chars().enumerate() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap() as u8;
            }
        }
        Self::empty()
    }
}

impl From<&str> for ALN {
    fn from(value: &str) -> Self {
        ALN::from(value.to_string())
    }
}