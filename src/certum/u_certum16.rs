use super::super::utils;
use utils::f64_split;
use utils::u64_to_u8_round;

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
    /// Convert an 8-bit Certum to a 64-bit Float
    fn from(value: uc16) -> Self {
        let (int, frc) = value.components();
        let float_frc = (frc as f64) / 256f64; // MSB-Shifted fraction / 2^Bits
        (int as f64) + float_frc // Add integer and fraction
    }
}

impl From<&uc16> for f64 {
    /// Convert an 8-bit Certum to a 64-bit Float
    fn from(value: &uc16) -> Self {
        f64::from(*value)
    }
}

impl From<uc16> for f32 {
    /// Convert an 8-bit Certum to a 32-bit Float
    fn from(value: uc16) -> Self {
        f64::from(value) as f32
    }
}

impl From<&uc16> for f32 {
    /// Convert an 8-bit Certum to a 32-bit Float
    fn from(value: &uc16) -> Self {
        f64::from(*value) as f32
    }
}


impl From<u8> for uc16 {
    /// Convert an 8-bit UInt to an 8-bit Certum
    fn from(bits: u8) -> Self {
        uc16 { bits }
    }
}

impl From<&u8> for uc16 {
    /// Convert an 8-bit UInt to an 8-bit Certum
    fn from(value: &u8) -> Self {
        uc16::from(*value)
    }
}

impl From<f32> for uc16 {
    /// Convert a 32-bit Float to an 8-bit Certum
    fn from(val: f32) -> Self {
        uc16::from(val as f64)
    }
}

impl From<f64> for uc16 {
    /// Convert a 64-bit Float to an 8-bit Certum
    fn from(val: f64) -> Self {
        let (_sgn, int, frc) = f64_split(val);
        // Combine integer and fraction parts
        // 8 bits - 2 int bits = 6 bit shifts
        // 8 bits - 6 bits = 2 bit shifts
        let bits = ((int as u8) << 6) | u64_to_u8_round(frc >> 2);
        uc16 { bits }
    }
}

impl uc16 {
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