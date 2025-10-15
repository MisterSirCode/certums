#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// Define a generic 128-bit Signed Certum
pub struct c128 {
    /// The raw bits of the certum
    /// 
    /// 1 Sign bit, 5 Integer bits, 122 Fraction bits
    pub bits: u128
}

impl c128 {
    /// Minimum value for a 128-bit Certum.
    /// 
    /// Decimal: -31.99999999999999999999999999999999999981192090386843399872500215404444069154901351091646599655859972699545323848724365234375
    pub const MIN: c128 = c128 { bits: 0x80000000000000000000000000000000 };
    /// Maximum value for a 128-bit Certum.
    /// 
    /// Decimal: 31.99999999999999999999999999999999999981192090386843399872500215404444069154901351091646599655859972699545323848724365234375
    pub const MAX: c128 = c128 { bits: 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF };
    /// Minimum value as a 128-bit Float
    pub const MINF: f64 = -32f64;
    /// Maximum value as a 128-bit Float
    pub const MAXF: f64 = 32f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.141592653589793115997963468544185161590576171875
    pub const PI: c128 = c128 { bits: 0x1921FB54442D18000000000000000000 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.718281828459045090795598298427648842334747314453125
    pub const E: c128 = c128 { bits: 0x15BF0A8B145769000000000000000000 };

    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u128 {
        if self.bits & 0x80000000000000000000000000000000 == 0x80000000000000000000000000000000 { 1 }
        else { 0 }
    }

    /// Get the scalar sign of the current certum
    /// 
    /// 1 = positive, 0 = zero, -1 = negative
    pub fn sign(&self) -> i8 {
        if self.bits == 0 { 0 } // 0 Case
        else if self.bits & 0x80000000000000000000000000000000 == 0x80000000000000000000000000000000 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    /// Return the binary components of the current certum
    /// 
    /// (Scalar Sign, Integer Component, Fraction Component)
    pub fn components(&self) -> (i8, u128, u128) {
        let sgn = self.sign(); // Get a binary sign of the certum
        // 128 bits - 5 int bits = 123 bit shifts
        let int = (self.bits << 1) >> 123; // Cut off sign bit and order integer's smallest component as LSB
        // 1 sgn bit + 5 int bits = 6 bit shifts
        let frc = self.bits << 6; // Order fraction's largest component as MSB
        (sgn, int, frc)
    }

    /// Internal use only
    pub fn u64_round(val: u64) -> u64 {
        val
    }
}