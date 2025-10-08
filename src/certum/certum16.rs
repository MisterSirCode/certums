use super::super::utils;
use utils::f64_split;
use utils::u64_to_u16_round;

/// Define a generic 8-bit Signed Certum
#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
pub struct c16 {
    /// The raw bits of the certum
    /// 
    /// 1 Sign bit, 2 Integer bits, 13 Fraction bits
    pub bits: u16
}

impl From<c16> for f64 {
    /// Convert a 16-bit Certum to a 64-bit Float
    fn from(value: c16) -> Self {
        let (sgn, int, frc) = value.components();
        let float_frc = (frc as f64) / 65536f64; // MSB-Shifted fraction / 2^Bits
        ((int as f64) + float_frc) * sgn as f64 // Add integer and fraction, multiply sign
    }
}

impl From<&c16> for f64 {
    /// Convert a 16-bit Certum to a 64-bit Float
    fn from(value: &c16) -> Self {
        f64::from(*value)
    }
}

impl From<c16> for f32 {
    /// Convert a 16-bit Certum to a 32-bit Float
    fn from(value: c16) -> Self {
        f64::from(value) as f32
    }
}

impl From<&c16> for f32 {
    /// Convert a 16-bit Certum to a 32-bit Float
    fn from(value: &c16) -> Self {
        f64::from(*value) as f32
    }
}

impl From<u16> for c16 {
    /// Convert a 16-bit UInt to a 16-bit Certum
    fn from(bits: u16) -> Self {
        c16 { bits }
    }
}

impl From<&u16> for c16 {
    /// Convert a 16-bit UInt to a 16-bit Certum
    fn from(value: &u16) -> Self {
        c16::from(*value)
    }
}

impl From<f32> for c16 {
    /// Convert a 32-bit Float to a 16-bit Certum
    fn from(val: f32) -> Self {
        c16::from(val as f64)
    }
}

impl From<f64> for c16 {
    /// Convert a 64-bit Float to a 16-bit Certum
    fn from(val: f64) -> Self {
        let (sgn, int, frc) = f64_split(val);
        // Adjust sign to be on the opposite side of the bits
        // 16 bits - 1 sgn bit = 15 bit shifts
        let sign = (sgn as u16) << 15;
        // Combine integer and fraction parts
        // 16 bits - 1 sgn bit - 2 int bits = 13 bit shifts
        // 1 sgn bit + 2 int bits = 3 bit shifts
        let combined = ((int as u16) << 13) | u64_to_u16_round(frc >> 3);
        // Clamp off for sign and add sign bit
        // 0x7FFF = 2^(16 bits - 1) - 1
        let bits = sign | (combined & 0x7FFF);
        c16 { bits }
    }
}

impl c16 {
    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u16 {
        if self.bits & 0x8000 == 0x8000 { 1 }
        else { 0 }
    }

    /// Get the scalar sign of the current certum
    /// 
    /// 1 = positive, 0 = zero, -1 = negative
    pub fn sign(&self) -> i8 {
        if self.bits == 0 { 0 } // 0 Case
        else if self.bits & 0x8000 == 0x8000 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    /// Return the binary components of the current certum
    /// 
    /// (Scalar Sign, Integer Component, Fraction Component)
    pub fn components(&self) -> (i8, u16, u16) {
        let sgn = self.sign(); // Get a binary sign of the certum
        // 16 bits - 2 int bits = 14 bit shifts
        let int = (self.bits << 1) >> 14; // Cut off sign bit and order integer's smallest component as LSB
        // 1 sgn bit + 2 int bits = 3 bit shifts
        let frc = self.bits << 3; // Order fraction's largest component as MSB
        (sgn, int, frc)
    }
}