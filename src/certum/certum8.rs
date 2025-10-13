#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// Define a generic 8-bit Signed Certum
pub struct c8 {
    /// The raw bits of the certum
    /// 
    /// 1 Sign bit, 1 Integer bit, 6 Fraction bits
    pub bits: u8
}

impl c8 {
    /// Minimum value for an 8-bit Certum.
    /// 
    /// Decimal: -1.984375
    pub const MIN: c8 = c8 { bits: 0x80 };
    /// Maximum value for an 8-bit Certum.
    /// 
    /// Decimal: 1.984375
    pub const MAX: c8 = c8 { bits: 0x7F };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = -1.984375f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 1.984375f64;

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
        // 1 sgn bit + 1 int bit = 2 bit shifts
        let frc = self.bits << 2;
        (sgn, int, frc)
    }

    /// Clamp a u64 and round to a u8 properly.
    /// 
    /// Right-shift MSB to (64 - 9), carry case with + 1, right-shift MSB to make 8 bits. Clamp to u8
    pub fn u64_round(val: u64) -> u8 {
        ((val + 0x80000000000000) >> 56) as u8
    }
}