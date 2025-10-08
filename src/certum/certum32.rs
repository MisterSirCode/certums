use super::super::utils;
use utils::f64_split;
use utils::u64_to_u32_round;

/// Define a generic 8-bit Signed Certum
#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
pub struct c32 {
    /// The raw bits of the certum
    /// 
    /// 1 Sign bit, 3 Integer bits, 28 Fraction bits
    pub bits: u32
}

// Convert Certums to Floats

impl From<c32> for f64 {
    fn from(value: c32) -> Self {
        let (sgn, int, frc) = value.components();
        let float_frc = (frc as f64) / 268435456f64; // MSB-Shifted fraction / 2^Bits
        ((int as f64) + float_frc) * sgn as f64 // Add integer and fraction, multiply sign
    }
}

impl From<&c32> for f64 {
    fn from(value: &c32) -> Self {
        f64::from(*value)
    }
}

impl From<c32> for f32 {
    fn from(value: c32) -> Self {
        f64::from(value) as f32
    }
}

impl From<&c32> for f32 {
    fn from(value: &c32) -> Self {
        f64::from(*value) as f32
    }
}

// Create a Certum from a u32

impl From<u32> for c32 {
    fn from(bits: u32) -> Self {
        c32 { bits }
    }
}

impl From<&u32> for c32 {
    fn from(value: &u32) -> Self {
        c32::from(*value)
    }
}

impl From<f32> for c32 {
    /// Convert a 32-bit Float to a 16-bit Certum
    fn from(val: f32) -> Self {
        c32::from(val as f64)
    }
}

impl From<f64> for c32 {
    /// Convert a 64-bit Float to a 16-bit Certum
    fn from(val: f64) -> Self {
        let (sgn, int, frc) = f64_split(val);
        // Adjust sign to be on the opposite side of the bits
        // 32 bits - 1 sign bit = 31 bit shifts
        let sign = (sgn as u32) << 31;
        // Combine integer and fraction parts
        // 32 bits - 1 sign bit - 3 int bits = 28 bit shifts
        // 1 sign bit + 3 int bits = 4 bit shifts
        let combined = ((int as u32) << 28) | u64_to_u32_round(frc >> 4);
        // Clamp off for sign and add sign bit
        // 0x7FFFFFFF = 2^(32 bits - 1) - 1
        let bits = sign | (combined & 0x7FFFFFFF);
        c32 { bits }
    }
}

impl c32 {
    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u32 {
        if self.bits & 0x80000000 == 0x80000000 { 1 }
        else { 0 }
    }

    /// Get the scalar sign of the current certum
    /// 
    /// 1 = positive, 0 = zero, -1 = negative
    pub fn sign(&self) -> i8 {
        if self.bits == 0 { 0 } // 0 Case
        else if self.bits & 0x80000000 == 0x80000000 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    /// Return the binary components of the current certum
    /// 
    /// (Scalar Sign, Integer Component, Fraction Component)
    pub fn components(&self) -> (i8, u32, u32) {
        let sgn = self.sign(); // Get a binary sign of the certum
        // 32 bits - 3 int bits = 29 bit shifts
        let int = (self.bits << 1) >> 29; // Cut off sign bit and order integer's smallest component as LSB
        // 1 sign bit + 3 int bits = 4 bit shifts
        let frc = self.bits << 4; // Order fraction's largest component as MSB
        (sgn, int, frc)
    }
}