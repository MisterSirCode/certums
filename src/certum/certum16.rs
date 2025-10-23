#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// Define a generic 16-bit Signed Certum
pub struct c16 {
    /// The raw bits of the certum
    /// 
    /// 1 Sign bit, 2 Integer bits, 13 Fraction bits
    pub bits: u16
}

impl c16 {
    /// Minimum value in bits
    pub const MINB: u16 = 0x8000;
    /// Maximum value in bits
    pub const MAXB: u16 = 0x7FFF;
    /// Minimum value for a 16-bit Certum.
    /// 
    /// Decimal: -3.9998779296875
    pub const MIN: Self = Self { bits: Self::MINB };
    /// Maximum value for a 16-bit Certum.
    /// 
    /// Decimal: 3.9998779296875
    pub const MAX: Self = Self { bits: Self::MAXB };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = -4.0f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 3.9998779296875f64;
    /// Grain - Smallest possible absolute quantity of this type
    pub const GRN: Self = Self { bits: 0b1 };
    /// Archimedes' Constant - π
    /// 
    /// Decimal: 3.1414794921875
    /// Exact:   3.141
    /// 
    /// Digits of Accuracy: 4
    /// 
    pub const PI: Self = Self { bits: 0x6487 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.71826171875
    /// Exact:   2.7182
    /// 
    /// Digits of Accuracy: 5
    /// 
    pub const E: Self = Self { bits: 0x56FC };

    /// Get the sign bit of the current certum in the proper location
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn sign_inverter(&self) -> u16 {
        if self.bits & Self::MINB == Self::MINB { Self::MINB }
        else { Self::MAXB }
    }

    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u16 {
        if self.bits & Self::MINB == Self::MINB { 1 }
        else { 0 }
    }

    /// Get the scalar sign of the current certum
    /// 
    /// 1 = positive, 0 = zero, -1 = negative
    pub fn sign(&self) -> i8 {
        if self.bits == 0 { 0 } // 0 Case
        else if self.bits & Self::MINB == Self::MINB { -1 } // Match MSB - Negative
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

    /// Clamp a u64 and round to a u16 properly.
    /// 
    /// Right-shift MSB to (64 - 17), carry case with + 1, right-shift MSB to make 16 bits. Clamp to u16
    pub fn u64_round(val: u64) -> u16 {
        ((val + 0x800000000000) >> 48) as u16
    }

    /// Extract the bits as its signed counterpart
    pub fn as_signed_bits(&self) -> i16 {
        self.bits as i16
    }

    /// Print line to console with a name and bits
    pub fn log_bits(&self) {
        println!("0b{:016b}", self.bits);
    }

    /// Print line to console with a name and hexadecimal bits
    pub fn log_hex(&self) {
        println!("0x{:04X}", self.bits);
    }

    /// Print line to console with a name and float value
    pub fn log_value(&self) {
        println!("{:.32}", f64::from(self));
    }
}