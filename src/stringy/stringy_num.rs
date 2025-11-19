#![allow(dead_code)]

use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use std::cmp::{Eq, PartialEq, Ordering};

/// Char Utilities
struct CU { }
impl CU {
    pub fn gt(lhs: &char, rhs: &char) {
        let lhs_n = *lhs as u32;
    }
}

#[derive(Clone, Debug)]
/// A Stringy a type of dynamic, arbitary-length number that is described by a string of numerics rather then binary.
/// 
/// This type is slow and only meant for precision conversions and displaying
pub struct Stringy {
    pub sgn: bool,
    pub int: String,
    pub frc: String,
    pub dec: bool
}

impl Add for Stringy {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let comp = usize::max(self.len(), rhs.len()); // Iterate based off the widest number
        for i in 0..=(comp - 1) {
            
        }
        self
    }
}

impl Stringy {
    /// Create a new Stringy with automatic input filtering and validation. 
    /// Inefficient and slow, but will work for input conversion.
    /// 
    /// Returns the tuple (Stringy, Bool). The boolean represents the validator, and will always be true if the input was successful.
    /// If the tuple boolean is false, your input was invalid in some way and no number was successfully pulled
    pub fn create(raw: String) -> (Self, bool) {
        let sgn: bool = raw.starts_with('-');
        let mut has_dp: bool = false; // Has a decimal point / Is a decimal number
        let mut valid_int: String = Default::default();
        let mut valid_frc: String = Default::default();
        let mut has_num: bool = false;
        for c in raw.chars() {
            if c.is_ascii_digit() {
                has_num = true;
                if has_dp { 
                    valid_frc.push(c); // Push to fractional
                } else { 
                    if (valid_int.len() > 1) | (c != '0') { // Ignore leading zeroes.
                        valid_int.push(c); // Push to integer
                    }
                }
            } else if c == '.' { has_dp = true; } // flip to the fractional side once the first decimal point is found
            // Reject any other character
        }
        while valid_frc.ends_with('0') { // Eliminate trailing zeroes.
            valid_frc.pop();
        }
        if valid_frc.len() == 0 { // Incase of a zero fraction, just interpret as an integer for simplicity
            has_dp = false;
        }
        (Self { sgn, int: valid_int, frc: valid_frc, dec: has_dp }, has_num)
    }

    /// Fast Integer. Creates a new Stringy and avoids input validation for the sake of efficiency.
    /// Cannot be used with decimals / fractional values.
    /// 
    /// Can be volatile if misused
    pub fn fast_int(int: String) -> Self {
        let sgn: bool = int.starts_with('-');
        let mut rec = int;
        if sgn { rec = }
        Self { int, frc: "".to_string(), dec: false }
    }

    /// Fast Fraction. Creates a new Stringy and avoids input validation for the sake of efficiency.
    /// Cannot be used with integer values.
    /// 
    /// Can be volatile if misused
    pub fn fast_frc(frc: String) -> Self {
        Self { int: "".to_string(), frc, dec: true }
    }

    /// Fast Decimal. Creates a new Stringy and avoids input validation for the sake of efficiency.
    /// Accepts an integer and fraction component.
    /// 
    /// Can be volatile if misused
    pub fn fast_dec(int: String, frc: String) -> Self {
        Self { int, frc, dec: true }
    }

    /// Return the length of the strings within the Stringy.
    /// Only accounts for the digits, and not a decimal point
    pub fn len(&self) -> usize {
        self.int.len() + self.frc.len()
    }
}

