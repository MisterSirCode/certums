use super::super::utils;
use utils::f32_split;
use utils::u32_to_u8_round;

/// Define a generic 8-bit Signed Certum
#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
pub struct c8 {
    /// The raw bits of the certum
    /// 
    /// 1 Sign bit, 1 Integer bit, 6 Fraction bits
    pub bits: u8
}

// Convert Certums to Floats

impl From<c8> for f64 {
    fn from(value: c8) -> Self {
        let (sgn, int, frc) = value.components();
        let float_frc = (frc as f64) / 256f64; // MSB-Shifted fraction / 2^Bits
        ((int as f64) + float_frc) * sgn as f64 // Add integer and fraction, multiply sign
    }
}

impl From<&c8> for f64 {
    fn from(value: &c8) -> Self {
        f64::from(*value)
    }
}

impl From<c8> for f32 {
    fn from(value: c8) -> Self {
        f64::from(value) as f32
    }
}

impl From<&c8> for f32 {
    fn from(value: &c8) -> Self {
        f64::from(*value) as f32
    }
}

// Create a Certum from a u8

impl From<u8> for c8 {
    fn from(bits: u8) -> Self {
        c8 { bits }
    }
}

impl From<&u8> for c8 {
    fn from(value: &u8) -> Self {
        c8::from(*value)
    }
}

// Convert Floats to Certums

impl From<f32> for c8 {
    fn from(val: f32) -> Self {
        let (sgn, int, frc) = f32_split(val);
        // Adjust sign to be on the opposite side of the bits
        // 8 bits - 1 sign bit = 7 bit shifts
        let sign = sgn << 7;
        // Combine integer and fraction parts
        // 8 bits - 1 sign bit - 1 int bit = 6 bit shifts
        // 8 bits - 6 bits = 2 bit shifts
        let combined = ((int as u8) << 6) | u32_to_u8_round(frc >> 2);
        // Clamp off for sign and add sign bit
        let bits = sign | (combined & 0x7F);
        c8 { bits }
    }
}

impl From<f64> for c8 {
    fn from(val: f64) -> Self {
        c8::from(val as f32) // f32 is beyond c8 precision, dont duplicate code
    }
}

impl c8 {
    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u8 {
        if self.bits & 0x80 == 0x80 { 1 }
        else { 0 }
    }

    /// Get the scalar sign of the current certum
    /// 
    /// 1 = positive, 0 = zero, -1 = negative
    pub fn sign(&self) -> i8 {
        if self.bits == 0 { 0 } // 0 Case
        else if self.bits & 0x80 == 0x80 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    /// Return the binary components of the current certum
    /// 
    /// (Scalar Sign, Integer Component, Fraction Component)
    pub fn components(&self) -> (i8, u8, u8) {
        // Get a binary sign of the certum
        let sgn = self.sign();
        // Cut off sign bit and order ints smallest component as LSB
        // 8 bits - 1 int bit = 7 bit shifts
        let int = (self.bits << 1) >> 7;
        // Order fraction's largest component as MSB
        // 1 sign bit + 1 int bit = 2 bit shifts
        let frc = self.bits << 2;
        (sgn, int, frc)
    }
}