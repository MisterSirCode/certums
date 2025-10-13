#![allow(dead_code)]

use super::super::utils;
use utils::f64_split;
use utils::u64_to_u16_round;

/// Define a generic 8-bit Signed Certum
#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
pub struct uc16 {
    /// The raw bits of the certum
    /// 
    /// 3 Integer bits, 13 Fraction bits
    pub bits: u16
}

impl From<uc16> for f64 {
    /// Convert an 16-bit Certum to a 64-bit Float
    fn from(value: uc16) -> Self {
        let (int, frc) = value.components();
        let float_frc = (frc as f64) / 65536f64; // MSB-Shifted fraction / 2^Bits
        (int as f64) + float_frc // Add integer and fraction
    }
}

impl From<&uc16> for f64 {
    /// Convert an 16-bit Certum to a 64-bit Float
    fn from(value: &uc16) -> Self {
        f64::from(*value)
    }
}

impl From<uc16> for f32 {
    /// Convert an 16-bit Certum to a 32-bit Float
    fn from(value: uc16) -> Self {
        f64::from(value) as f32
    }
}

impl From<&uc16> for f32 {
    /// Convert an 16-bit Certum to a 32-bit Float
    fn from(value: &uc16) -> Self {
        f64::from(*value) as f32
    }
}


impl From<u16> for uc16 {
    /// Convert an 16-bit UInt to an 16-bit Certum
    fn from(bits: u16) -> Self {
        uc16 { bits }
    }
}

impl From<&u16> for uc16 {
    /// Convert an 16-bit UInt to an 16-bit Certum
    fn from(value: &u16) -> Self {
        uc16::from(*value)
    }
}

impl From<f32> for uc16 {
    /// Convert a 32-bit Float to an 16-bit Certum
    /// 
    /// Converting a float to an unsigned certum will truncate signs
    fn from(val: f32) -> Self {
        uc16::from(val as f64)
    }
}

impl From<f64> for uc16 {
    /// Convert a 64-bit Float to an 16-bit Certum
    /// 
    /// Converting a float to an unsigned certum will truncate signs
    fn from(val: f64) -> Self {
        let (_sgn, int, frc) = f64_split(val.clamp(uc16::MINF, uc16::MAXF));
        // Combine integer and fraction parts
        // 16 bits - 3 int bits = 13 bit shifts
        // 16 bits - 13 frc bits = 3 bit shifts
        let bits = ((int as u16) << 13) | u64_to_u16_round(frc >> 3);
        uc16 { bits }
    }
}

impl uc16 {
    /// Minimum value for a 16-bit Unsigned Certum.
    /// 
    /// Decimal: 0
    pub const MIN: uc16 = uc16 { bits: 0 };
    /// Maximum value for a 16-bit Unsigned Certum.
    /// 
    /// Decimal: 7.9998779296875
    pub const MAX: uc16 = uc16 { bits: 0xFFFF };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = 0f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 7.9998779296875f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.1416015625
    pub const PI: uc16 = uc16 { bits: 0x6488 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.71826171875
    pub const E: uc16 = uc16 { bits: 0x56FC };

    /// Return the binary components of the current certum
    /// 
    /// (Integer Component, Fraction Component)
    pub fn components(&self) -> (u16, u16) {
        // Order ints smallest component as LSB
        // 16 bits - 3 int bits = 13 bit shifts
        let int = self.bits >> 13;
        // Order fraction's largest component as MSB
        // 16 bits - 13 frc bits = 3 bit shifts
        let frc = self.bits << 3;
        (int, frc)
    }
}