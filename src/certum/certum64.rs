#![allow(dead_code)]

use super::super::utils;
use utils::f64_split;

/// Define a generic 8-bit Signed Certum
#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
pub struct c64 {
    /// The raw bits of the certum
    /// 
    /// 1 Sign bit, 4 Integer bits, 59 Fraction bits
    pub bits: u64
}

impl From<c64> for f64 {
    /// Convert a 64-bit Certum to a 64-bit Float
    fn from(value: c64) -> Self {
        let (sgn, int, frc) = value.components();
        let float_frc = (frc as f64) / 18446744073709551616f64; // MSB-Shifted fraction / 2^Bits
        ((int as f64) + float_frc) * sgn as f64 // Add integer and fraction, multiply sign
    }
}

impl From<&c64> for f64 {
    /// Convert a 64-bit Certum to a 64-bit Float
    fn from(value: &c64) -> Self {
        f64::from(*value)
    }
}

impl From<c64> for f32 {
    /// Convert a 64-bit Certum to a 32-bit Float
    fn from(value: c64) -> Self {
        f64::from(value) as f32
    }
}

impl From<&c64> for f32 {
    /// Convert a 64-bit Certum to a 32-bit Float
    fn from(value: &c64) -> Self {
        f64::from(*value) as f32
    }
}

impl From<u64> for c64 {
    /// Convert a 32-bit UInt to a 64-bit Certum
    fn from(bits: u64) -> Self {
        c64 { bits }
    }
}

impl From<&u64> for c64 {
    /// Convert a 32-bit UInt to a 64-bit Certum
    fn from(value: &u64) -> Self {
        c64::from(*value)
    }
}

impl From<f32> for c64 {
    /// Convert a 32-bit Float to a 64-bit Certum
    fn from(val: f32) -> Self {
        c64::from(val as f64)
    }
}

impl From<f64> for c64 {
    /// Convert a 64-bit Float to a 64-bit Certum
    fn from(val: f64) -> Self {
        let (sgn, int, frc) = f64_split(val.clamp(c64::MINF, c64::MAXF));
        // Adjust sign to be on the opposite side of the bits
        // 64 bits - 1 sgn bit = 63 bit shifts
        let sign = (sgn as u64) << 63;
        // Combine integer and fraction parts
        // 64 bits - 1 sgn bit - 4 int bits = 59 bit shifts
        // 1 sgn bit + 4 int bits = 5 bit shifts
        let combined = ((int as u64) << 59) | (frc >> 5);
        // Clamp off for sign and add sign bit
        // 0x7FFFFFFFFFFFFFFF = 2^(64 bits - 1) - 1
        let bits = sign | (combined & 0x7FFFFFFFFFFFFFFF);
        c64 { bits }
    }
}

impl c64 {
    /// Minimum value for a 64-bit Certum.
    /// 
    /// Decimal: -15.99999999999999999826527652402319290558807551860809326171875
    const MIN: c64 = c64 { bits: 0x8000000000000000 };
    /// Maximum value for a 64-bit Certum.
    /// 
    /// Decimal: 15.99999999999999999826527652402319290558807551860809326171875
    const MAX: c64 = c64 { bits: 0x7FFFFFFFFFFFFFFF };
    /// Minimum value as a 64-bit Float
    const MINF: f64 = -16f64;
    /// Maximum value as a 64-bit Float
    const MAXF: f64 = 16f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.141592653589793115997963468544185161590576171875
    const PI: c64 = c64 { bits: 0x1921FB54442D1800 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.718281828459045090795598298427648842334747314453125
    const E: c64 = c64 { bits: 0x15BF0A8B14576900 };

    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u64 {
        if self.bits & 0x8000000000000000 == 0x8000000000000000 { 1 }
        else { 0 }
    }

    /// Get the scalar sign of the current certum
    /// 
    /// 1 = positive, 0 = zero, -1 = negative
    pub fn sign(&self) -> i8 {
        if self.bits == 0 { 0 } // 0 Case
        else if self.bits & 0x8000000000000000 == 0x8000000000000000 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    /// Return the binary components of the current certum
    /// 
    /// (Scalar Sign, Integer Component, Fraction Component)
    pub fn components(&self) -> (i8, u64, u64) {
        let sgn = self.sign(); // Get a binary sign of the certum
        // 64 bits - 4 int bits = 60 bit shifts
        let int = (self.bits << 1) >> 60; // Cut off sign bit and order integer's smallest component as LSB
        // 1 sgn bit + 4 int bits = 5 bit shifts
        let frc = self.bits << 5; // Order fraction's largest component as MSB
        (sgn, int, frc)
    }
}