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
    /// Minimum value in bits
    pub const MINB: u32 = 0x80000000;
    /// Maximum value in bits
    pub const MAXB: u32 = 0x7FFFFFFF;
    /// Minimum value for a 32-bit Certum.
    /// 
    /// Decimal: -7.9999999962747097015380859375
    pub const MIN: Self = Self { bits: Self::MINB };
    /// Maximum value for a 32-bit Certum.
    /// 
    /// Decimal: 7.9999999962747097015380859375
    pub const MAX: Self = Self { bits: Self::MAXB };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = -8.0f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 7.99999999627471f64;
    /// Grain - Smallest possible absolute quantity of this type
    pub const GRN: Self = Self { bits: 0b1 };
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.1415926516056060791015625
    /// Exact:   3.14159265
    /// 
    /// Digits of Accuracy: 9
    /// 
    pub const PI: Self = Self { bits: 0x3243F6A8 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.718281827867031097412109375
    /// Exact:   2.71828182
    /// 
    /// Digits of Accuracy: 9
    /// 
    pub const E: Self = Self { bits: 0x2B7E1516 };

    /// Get the sign bit of the current certum in the proper location
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn sign_inverter(&self) -> u32 {
        if self.bits & Self::MINB == Self::MINB { Self::MINB }
        else { Self::MAXB }
    }

    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u32 {
        if self.bits & Self::MINB == Self::MINB { 1 }
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

    /// Print line to console with a name and bits
    pub fn log_bits(&self) {
        println!("0b{:032b}", self.bits);
    }

    /// Print line to console with a name and hexadecimal bits
    pub fn log_hex(&self) {
        println!("0x{:08X}", self.bits);
    }

    /// Print line to console with a name and float value
    pub fn log_value(&self) {
        println!("0b{:.32}", f64::from(self));
    }
}