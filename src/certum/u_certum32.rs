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
        let (_sgn, int, frc) = f64_split(val.clamp(uc32::MINF, uc32::MAXF));
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
    pub const MIN: uc32 = uc32 { bits: 0 };
    /// Maximum value for a 32-bit Unsigned Certum.
    /// 
    /// Decimal: 15.9999999962747097015380859375
    pub const MAX: uc32 = uc32 { bits: 0xFFFFFFFF };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = 0f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 15.99999999627471f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.1415926553308963775634765625
    pub const PI: uc32 = uc32 { bits: 0x3243F6A9 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.718281827867031097412109375
    pub const E: uc32 = uc32 { bits: 0x2B7E1516 };

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