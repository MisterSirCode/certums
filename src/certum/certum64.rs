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
    /// Bits before the decimal point
    pub const DEC: u64 = 5;
    /// Bits after the decimal point
    pub const FRC: u64 = 59;
    /// Literal bits after the decimal point. 2 ^ FRC
    pub const FRCPOW: u64 = 576460752303423488;
    /// Minimum value in bits
    pub const MINB: u64 = 0x8000000000000000;
    /// Maximum value in bits
    pub const MAXB: u64 = 0x7FFFFFFFFFFFFFFF;
    /// Minimum value for a 64-bit Certum.
    /// 
    /// Decimal: -15.99999999999999999826527652402319290558807551860809326171875
    pub const MIN: Self = Self { bits: Self::MINB };
    /// Maximum value for a 64-bit Certum.
    /// 
    /// Decimal: 15.99999999999999999826527652402319290558807551860809326171875
    pub const MAX: Self = Self { bits: Self::MAXB };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = -16f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 16f64;
    /// Grain - Smallest possible absolute quantity of this type
    pub const GRN: Self = Self { bits: 0b1 };
    /// One - The certum equivalent of integer 1
    pub const ONE: u64 = 576460752303423489;
    /// Archimedes' Constant - π
    /// 
    /// Decimal: 3.1415926535897932374286067869206817704252898693084716796875
    /// Exact:   3.14159265358979323
    /// 
    /// Digits of Accuracy: 18
    /// 
    pub const PI: Self = Self { bits: 0x1921FB54442D1846 };
    /// Eulers' Number - e
    /// 
    /// Decimal: 2.71828182845904523477764680450263767852447926998138427734375
    /// Exact:   2.71828182845904523
    /// 
    /// Digits of Accuracy: 18
    /// 
    pub const E: Self = Self { bits: 0x15BF0A8B14576953 };

    /// Get a new certum from raw bits
    pub fn of(bits: u64) -> Self {
        Self { bits }
    }
    
    /// Get the sign bit of the current certum in the proper location
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn sign_inverter(&self) -> u64 {
        if self.bits & Self::MINB == Self::MINB { Self::MINB }
        else { Self::MAXB }
    }

    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u64 {
        if self.bits & Self::MINB == Self::MINB { 1 }
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

    /// Internal use only
    pub fn u64_round(val: u64) -> u64 {
        val
    }

    /// Extract the bits as its signed counterpart
    pub fn as_signed_bits(&self) -> i64 {
        self.bits as i64
    }

    /// Print line to console with a name and bits
    pub fn log_bits(&self) {
        println!("0b{:064b}", self.bits);
    }

    /// Print line to console with a name and hexadecimal bits
    pub fn log_hex(&self) {
        println!("0x{:016X}", self.bits);
    }

    /// Print line to console with a name and float value
    pub fn log_value(&self) {
        println!("{:.32}", f64::from(self));
    }
}