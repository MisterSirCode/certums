#![allow(dead_code)]

use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};
use std::cmp::{Eq, PartialEq, Ordering};

pub struct BigInt {
    pub bits: Vec<u64>
}

#[derive(Clone, Debug)]
/// ALN - Arbitrary Length Number
/// 
/// This type is slow and only meant for precision conversions and displaying
pub struct ALN {
    pub sgn: bool,
    pub int: Vec<u8>,
    pub frc: Vec<u8>,
}

impl ALN {
    /// Create an empty ALN
    pub fn empty() -> Self {
        Self { sgn: false, int: Vec::new(), frc: Vec::new() }
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
        usize
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
        // let digit_count: usize = int.len();
        let mut int_vec: Vec<u8> = Vec::new();
        int_vec.push(0);
        let mut index = 0;
        for (i, c) in int.chars().enumerate() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap() as u8;
                let (mut res, mut carry) = int_vec[index].carrying_mul(10, digit);
                if carry > 0 {
                    index = index + 1;
                    if (int_vec.len() - 1) < index {
                        int_vec.push(0);
                    }
                    int_vec[index] = digit;
                } else {
                    int_vec[index] = res;
                }
                println!("{:} - res: {:} - carry: {:}", digit, res, carry);
                print!("Vec State: ");
                let mut bits: u64 = 0; 
                for item in &int_vec {
                    print!("{:} ", item);
                    bits = (bits << 3) + (bits << 1) + (*item as u64);
                }
                // println!("");
                // println!("Raw Bits: {:064b}", bits);
                // println!("Raw Int: {:}", bits);
                println!("\n");
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