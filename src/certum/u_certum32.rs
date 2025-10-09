#![allow(dead_code)]

use super::super::utils;
use utils::f64_split;
use utils::u64_to_u32_round;

/// Define a generic 8-bit Signed Certum
#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
pub struct uc32 {
    /// The raw bits of the certum
    /// 
    /// 4 Integer bits, 28 Fraction bits
    pub bits: u32
}

impl From<uc32> for f64 {
    /// Convert an 32-bit Certum to a 64-bit Float
    fn from(value: uc32) -> Self {
        let (int, frc) = value.components();
        let float_frc = (frc as f64) / 4294967296f64; // MSB-Shifted fraction / 2^Bits
        (int as f64) + float_frc // Add integer and fraction
    }
}

impl From<&uc32> for f64 {
    /// Convert an 32-bit Certum to a 64-bit Float
    fn from(value: &uc32) -> Self {
        f64::from(*value)
    }
}

impl From<uc32> for f32 {
    /// Convert an 32-bit Certum to a 32-bit Float
    fn from(value: uc32) -> Self {
        f64::from(value) as f32
    }
}

impl From<&uc32> for f32 {
    /// Convert an 32-bit Certum to a 32-bit Float
    fn from(value: &uc32) -> Self {
        f64::from(*value) as f32
    }
}


impl From<u32> for uc32 {
    /// Convert an 32-bit UInt to an 32-bit Certum
    fn from(bits: u32) -> Self {
        uc32 { bits }
    }
}

impl From<&u32> for uc32 {
    /// Convert an 32-bit UInt to an 32-bit Certum
    fn from(value: &u32) -> Self {
        uc32::from(*value)
    }
}

impl From<f32> for uc32 {
    /// Convert a 32-bit Float to an 32-bit Certum
    /// 
    /// Converting a float to an unsigned certum will truncate signs
    fn from(val: f32) -> Self {
        uc32::from(val as f64)
    }
}

impl From<f64> for uc32 {
    /// Convert a 64-bit Float to an 32-bit Certum
    /// 
    /// Converting a float to an unsigned certum will truncate signs
    fn from(val: f64) -> Self {
        let (_sgn, int, frc) = f64_split(val);
        // Combine integer and fraction parts
        // 32 bits - 4 int bits = 28 bit shifts
        // 32 bits - 28 frc bits = 4 bit shifts
        let bits = ((int as u32) << 28) | u64_to_u32_round(frc >> 4);
        uc32 { bits }
    }
}

impl uc32 {
    /// Minimum value for a 32-bit Unsigned Certum.
    /// 
    /// Decimal: 0
    const MIN: uc32 = uc32 { bits: 0 };
    /// Maximum value for a 32-bit Unsigned Certum.
    /// 
    /// Decimal: 15.9999999962747097015380859375
    const MAX: uc32 = uc32 { bits: 0xFFFFFFFF };
    /// Archimede's Constant - Pi
    /// 
    /// Decimal: 3.1415926553308963775634765625
    const PI: uc32 = uc32 { bits: 0x3243F6A9 };

    /// Return the binary components of the current certum
    /// 
    /// (Integer Component, Fraction Component)
    pub fn components(&self) -> (u32, u32) {
        // Order ints smallest component as LSB
        // 32 bits - 4 int bits = 28 bit shifts
        let int = self.bits >> 28;
        // Order fraction's largest component as MSB
        // 32 bits - 28 frc bits = 4 bit shifts
        let frc = self.bits << 4;
        (int, frc)
    }
}