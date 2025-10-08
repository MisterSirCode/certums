use super::super::utils;
use utils::f32_split;
use utils::f64_split;
use utils::u32_to_u16_round;

/// Define a generic 8-bit Signed Certum
#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
pub struct c16 {
    /// The raw bits of the certum
    /// 
    /// 1 Sign bit, 1 Whole Number bit, 6 Fractional bits
    pub bits: u16
}

// impl From<c16> for f32 {
//     fn from(value: c16) -> Self {
//         c16::as_float(&value) as f32
//     }
// }

// impl From<&c16> for f32 {
//     fn from(value: &c16) -> Self {
//         f32::from(*value)
//     }
// }

// Convert Certums to Floats

impl From<c16> for f64 {
    fn from(value: c16) -> Self {
        let (sgn, int, frc) = value.components();
        let float_frc = (frc as f64) / 256f64; // MSB-Shifted fractional / 2^Bits
        ((int as f64) + float_frc) * sgn as f64 // Add integer and fractional, multiply sign
    }
}

impl From<&c16> for f64 {
    fn from(value: &c16) -> Self {
        f64::from(*value)
    }
}

impl From<c16> for f32 {
    fn from(value: c16) -> Self {
        f64::from(value) as f32
    }
}

impl From<&c16> for f32 {
    fn from(value: &c16) -> Self {
        f64::from(*value) as f32
    }
}

// Create a Certum from a u16

impl From<u16> for c16 {
    fn from(bits: u16) -> Self {
        c16 { bits }
    }
}

impl From<&u16> for c16 {
    fn from(value: &u16) -> Self {
        c16::from(*value)
    }
}

// Convert Floats to Certums

impl From<f32> for c16 {
    fn from(val: f32) -> Self {
        let (sgn, int, frc) = utils::f32_split(val);
        let sign = (sgn as u16) << 15; // Adjust sign to be on the opposite site of the bits
        let combined = ((int as u16) << 6) | u32_to_u16_round(frc >> 2); // Combine integer and fractional parts
        let bits = sign | (combined & 0x7FFF); // Add sign bit
        c16 { bits } // clamp off for sign
    }
}

impl From<f64> for c16 {
    fn from(val: f64) -> Self {
        c16::from(val as f32) // f32 is beyond c16 precision, dont duplicate code
    }
}

impl c16 {
    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u16 {
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
    pub fn components(&self) -> (i8, u16, u16) {
        let sgn = self.sign(); // Get a binary sign of the certum
        let int = (self.bits << 1) >> 7; // Order ints smallest component as LSB
        let frc = self.bits << 2; // Order fraction's largest component as MSB
        (sgn, int, frc)
    }
}