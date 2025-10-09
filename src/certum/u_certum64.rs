#![allow(dead_code)]

use super::super::utils;
use utils::f64_split;

/// Define a generic 8-bit Signed Certum
#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
pub struct uc64 {
    /// The raw bits of the certum
    /// 
    /// 5 Integer bits, 59 Fraction bits
    pub bits: u64
}

impl From<uc64> for f64 {
    /// Convert an 64-bit Certum to a 64-bit Float
    fn from(value: uc64) -> Self {
        let (int, frc) = value.components();
        let float_frc = (frc as f64) / 18446744073709551616f64; // MSB-Shifted fraction / 2^Bits
        (int as f64) + float_frc // Add integer and fraction
    }
}

impl From<&uc64> for f64 {
    /// Convert an 64-bit Certum to a 64-bit Float
    fn from(value: &uc64) -> Self {
        f64::from(*value)
    }
}

impl From<uc64> for f32 {
    /// Convert an 64-bit Certum to a 32-bit Float
    fn from(value: uc64) -> Self {
        f64::from(value) as f32
    }
}

impl From<&uc64> for f32 {
    /// Convert an 64-bit Certum to a 32-bit Float
    fn from(value: &uc64) -> Self {
        f64::from(*value) as f32
    }
}


impl From<u64> for uc64 {
    /// Convert an 64-bit UInt to an 64-bit Certum
    fn from(bits: u64) -> Self {
        uc64 { bits }
    }
}

impl From<&u64> for uc64 {
    /// Convert an 64-bit UInt to an 64-bit Certum
    fn from(value: &u64) -> Self {
        uc64::from(*value)
    }
}

impl From<f32> for uc64 {
    /// Convert a 32-bit Float to an 64-bit Certum
    /// 
    /// Converting a float to an unsigned certum will truncate signs
    fn from(val: f32) -> Self {
        uc64::from(val as f64)
    }
}

impl From<f64> for uc64 {
    /// Convert a 64-bit Float to an 64-bit Certum
    /// 
    /// Converting a float to an unsigned certum will truncate signs
    fn from(val: f64) -> Self {
        let (_sgn, int, frc) = f64_split(val.clamp(uc64::MINF, uc64::MAXF));
        // Combine integer and fraction parts
        // 64 bits - 5 int bits = 59 bit shifts
        // 64 bits - 59 frc bits = 5 bit shifts
        let bits = ((int as u64) << 59) | (frc >> 5);
        uc64 { bits }
    }
}

impl uc64 {
    /// Minimum value for a 64-bit Unsigned Certum.
    /// 
    /// Decimal: 0
    const MIN: uc64 = uc64 { bits: 0 };
    /// Maximum value for a 64-bit Unsigned Certum.
    /// 
    /// Decimal: 31.99999999999999999826527652402319290558807551860809326171875
    const MAX: uc64 = uc64 { bits: 0xFFFFFFFFFFFFFFFF };
    /// Minimum value as a 64-bit Float
    const MINF: f64 = 0f64;
    /// Maximum value as a 64-bit Float
    const MAXF: f64 = 32f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.141592653589793115997963468544185161590576171875
    const PI: uc64 = uc64 { bits: 0x1921FB54442D1800 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.718281828459045090795598298427648842334747314453125
    const E: uc64 = uc64 { bits: 0x15BF0A8B14576900 };

    /// Return the binary components of the current certum
    /// 
    /// (Integer Component, Fraction Component)
    pub fn components(&self) -> (u64, u64) {
        // Order ints smallest component as LSB
        // 64 bits - 5 int bits = 6 bit shifts
        let int = self.bits >> 59;
        // Order fraction's largest component as MSB
        // 64 bits - 59 frc bits = 5 bit shifts
        let frc = self.bits << 5;
        (int, frc)
    }
}