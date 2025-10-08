use super::super::utils;
use utils::f64_split;
use utils::u64_to_u8_round;

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
    /// Convert an 8-bit Certum to a 64-bit Float
    fn from(value: uc32) -> Self {
        let (int, frc) = value.components();
        let float_frc = (frc as f64) / 256f64; // MSB-Shifted fraction / 2^Bits
        (int as f64) + float_frc // Add integer and fraction
    }
}

impl From<&uc32> for f64 {
    /// Convert an 8-bit Certum to a 64-bit Float
    fn from(value: &uc32) -> Self {
        f64::from(*value)
    }
}

impl From<uc32> for f32 {
    /// Convert an 8-bit Certum to a 32-bit Float
    fn from(value: uc32) -> Self {
        f64::from(value) as f32
    }
}

impl From<&uc32> for f32 {
    /// Convert an 8-bit Certum to a 32-bit Float
    fn from(value: &uc32) -> Self {
        f64::from(*value) as f32
    }
}


impl From<u8> for uc32 {
    /// Convert an 8-bit UInt to an 8-bit Certum
    fn from(bits: u8) -> Self {
        uc32 { bits }
    }
}

impl From<&u8> for uc32 {
    /// Convert an 8-bit UInt to an 8-bit Certum
    fn from(value: &u8) -> Self {
        uc32::from(*value)
    }
}

impl From<f32> for uc32 {
    /// Convert a 32-bit Float to an 8-bit Certum
    fn from(val: f32) -> Self {
        uc32::from(val as f64)
    }
}

impl From<f64> for uc32 {
    /// Convert a 64-bit Float to an 8-bit Certum
    fn from(val: f64) -> Self {
        let (_sgn, int, frc) = f64_split(val);
        // Combine integer and fraction parts
        // 8 bits - 2 int bits = 6 bit shifts
        // 8 bits - 6 bits = 2 bit shifts
        let bits = ((int as u8) << 6) | u64_to_u8_round(frc >> 2);
        uc32 { bits }
    }
}

impl uc32 {
    /// Return the binary components of the current certum
    /// 
    /// (Integer Component, Fraction Component)
    pub fn components(&self) -> (u8, u8) {
        // Order ints smallest component as LSB
        // 8 bits - 2 int bits = 6 bit shifts
        let int = self.bits >> 6;
        // Order fraction's largest component as MSB
        // 8 bits - 6 frc bits = 2 bit shifts
        let frc = self.bits << 2;
        (int, frc)
    }
}