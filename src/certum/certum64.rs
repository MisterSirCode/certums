#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// Define a generic 64-bit Signed Certum
pub struct c64 {
    /// The raw bits of the certum
    /// 
    /// 1 Sign bit, 4 Integer bits, 59 Fraction bits
    pub bits: u64
}

impl c64 {
    /// Minimum value for a 64-bit Certum.
    /// 
    /// Decimal: -15.99999999999999999826527652402319290558807551860809326171875
    pub const MIN: c64 = c64 { bits: 0x8000000000000000 };
    /// Maximum value for a 64-bit Certum.
    /// 
    /// Decimal: 15.99999999999999999826527652402319290558807551860809326171875
    pub const MAX: c64 = c64 { bits: 0x7FFFFFFFFFFFFFFF };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = -16f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 16f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.141592653589793115997963468544185161590576171875
    pub const PI: c64 = c64 { bits: 0x1921FB54442D1800 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.718281828459045090795598298427648842334747314453125
    pub const E: c64 = c64 { bits: 0x15BF0A8B14576900 };

    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u64 {
        if self.bits & 0x8000000000000000 == 0x8000000000000000 { 1 }
        else { 0 }
    }

    /// Get the scalar sign of the current certum
    /// 
    /// 1 = positive, 0 = zero, -1 = negative
    pub fn sign(&self) -> i8 {
        if self.bits == 0 { 0 } // 0 Case
        else if self.bits & 0x8000000000000000 == 0x8000000000000000 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    /// Return the binary components of the current certum
    /// 
    /// (Scalar Sign, Integer Component, Fraction Component)
    pub fn components(&self) -> (i8, u64, u64) {
        let sgn = self.sign(); // Get a binary sign of the certum
        // 64 bits - 4 int bits = 60 bit shifts
        let int = (self.bits << 1) >> 60; // Cut off sign bit and order integer's smallest component as LSB
        // 1 sgn bit + 4 int bits = 5 bit shifts
        let frc = self.bits << 5; // Order fraction's largest component as MSB
        (sgn, int, frc)
    }

    /// Internal Use Only
    pub fn u64_round(val: u64) -> u64 {
        val
    }
}