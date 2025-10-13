#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// Define a generic 32-bit Signed Certum
pub struct c32 {
    /// The raw bits of the certum
    /// 
    /// 1 Sign bit, 3 Integer bits, 28 Fraction bits
    pub bits: u32
}

impl c32 {
    /// Minimum value for a 32-bit Certum.
    /// 
    /// Decimal: -7.9999999962747097015380859375
    pub const MIN: c32 = c32 { bits: 0x80000000 };
    /// Maximum value for a 32-bit Certum.
    /// 
    /// Decimal: 7.9999999962747097015380859375
    pub const MAX: c32 = c32 { bits: 0x7FFFFFFF };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = -7.99999999627471f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 7.99999999627471f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.1415926553308963775634765625
    pub const PI: c32 = c32 { bits: 0x3243F6A9 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.718281827867031097412109375
    pub const E: c32 = c32 { bits: 0x2B7E1516 };

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
        // 1 sgn bit + 3 int bits = 4 bit shifts
        let frc = self.bits << 4; // Order fraction's largest component as MSB
        (sgn, int, frc)
    }

    /// Clamp a u64 and round to a u32 properly.
    /// 
    /// Right-shift MSB to (64 - 33), carry case with + 1, right-shift MSB to make 32 bits. Clamp to u32
    pub fn u64_round(val: u64) -> u32 {
        ((val + 0x80000000) >> 32) as u32
    }
}