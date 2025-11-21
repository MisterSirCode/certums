#![allow(dead_code)]

use std::{
    fmt::{Binary, Formatter, LowerHex, Result, UpperHex},
    iter,
    ops::{Add, Mul, Shl, Shr},
};


type udef = u8;

/// BigInt courtesy of @quelfth on the rust discord
#[derive(Clone, Debug)]
pub struct BigInt(Vec<udef>);

impl BigInt {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_be_bytes(bytes: &[udef]) -> Self {
        Self(bytes.iter().copied().rev().collect())
    }

    pub fn from_le_bytes(bytes: &[udef]) -> Self {
        Self(bytes.to_owned())
    }
}

impl<'a> Add<&'a Self> for BigInt {
    type Output = Self;
    fn add(mut self, rhs: &'a Self) -> Self::Output {
        let mut carry = false;
        // We need to make sure that the lhs has enough space to hold the result.
        // So if it is too short, then we will extend it with zeros.
        if self.0.len() < rhs.0.len() {
            self.0.resize(rhs.0.len(), 0);
        }

        // We will be modifying `self` through this iter, so we use `iter_mut`
        let lhs_iter = self.0.iter_mut();
        // `.copied()` to convert from `&u8` to `u8`
        // We chain `iter::repeat(0)` onto the end so that in case the rhs runs out of bytes,
        // it will effectively be padded out with zeros. `iter::repeat()` is an infinite iterator,
        // but zip will stop when either iterator runs out, and `lhs_iter` is finite.
        let rhs_iter = rhs.0.iter().copied().chain(iter::repeat(0));
        for (l, r) in lhs_iter.zip(rhs_iter) {
            (*l, carry) = l.carrying_add(r, carry);
        }

        // If there was an extra carry, we need to push a `1` to the end of the int.
        if carry {
            self.0.push(1);
        }
        self
    }
}

impl Mul<udef> for BigInt {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(mut self, rhs: udef) -> Self::Output {
        let mut carries = vec![0];
        for byte in &mut self.0 {
            let (res, carry) = byte.carrying_mul(rhs, 0);
            *byte = res;
            carries.push(carry);
        }
        self + &Self(carries)
    }
}

impl Shr<udef> for BigInt {
    type Output = Self;
    fn shr(mut self, rhs: udef) -> Self::Output {
        let bits = udef::BITS as udef;
        let (moves, shift) = (rhs / bits, rhs % bits);
        self.0.drain(0..(moves as usize)); // for shifts greater than bits, slice off entire bytes
        if shift != 0 {
            for i in 0..self.0.len() - 1 {
                self.0[i] = (self.0[i] >> shift) | (self.0[i + 1] << (bits - shift))
            }
        }
        self
    }
}

impl Shl<udef> for BigInt {
    type Output = Self;
    fn shl(mut self, rhs: udef) -> Self::Output {
        let bits = udef::BITS as udef;
        let (moves, shift) = (rhs / bits, rhs % bits);
        let depth = self.0.len() - 1;
        self.0.drain((depth-(moves as usize))..depth); // for shifts greater than bits, slice off entire bytes
        if shift != 0 {
            for i in 0..depth {
                let ti = depth - i - 1;
                //  | (self.0[i + 1] << (bits - shift))
                if ti > 0 {
                    self.0[ti] = (self.0[ti] << shift) | (self.0[ti - 1] >> (bits - shift))
                } else {
                    self.0[ti] = self.0[ti] << shift
                }
            }
        }
        self
    }
}

impl LowerHex for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut iter = self.0.iter().rev();
        if let Some(byte) = iter.next() {
            write!(f, "{byte:x}")?;
        }

        for &byte in iter {
            write!(f, "{byte:0wid$X}", wid = size_of::<udef>())?;
        }
        Ok(())
    }
}

impl UpperHex for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut iter = self.0.iter().rev();
        if let Some(byte) = iter.next() {
            write!(f, "{byte:X}")?;
        }

        for &byte in iter {
            write!(f, "{byte:0wid$X}", wid = size_of::<udef>())?;
        }
        Ok(())
    }
}

impl Binary for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut iter = self.0.iter().rev();
        if let Some(byte) = iter.next() {
            write!(f, "{byte:b}")?;
        }

        for &byte in iter {
            if byte > 0 {
                write!(f, "{byte:0wid$b}", wid = size_of::<udef>() * 8)?;
            }
        }
        Ok(())
    }
}

impl From<udef> for BigInt {
    fn from(value: udef) -> Self {
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
        let mut aln = Self::empty();
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
            if let Some(digit) = c.to_digit(10) {
                aln.int = (aln.int * 10) + &BigInt::from(digit as udef);
            }
            print!("Vec State: ");
            for item in &aln.int.0 {
                if item > &0 {
                    print!("{:} ", item);
                }
            }
            println!("");
            println!("00000000{:b} ", aln.int);
            // println!("{:b} ", (aln.int.clone() >> 3));
            println!("{:b} ", (aln.int.clone() << 3));
        }
        Self::empty()
    }
}

impl From<&str> for ALN {
    fn from(value: &str) -> Self {
        ALN::from(value.to_string())
    }
}