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
    /// Bits before the decimal point
    pub const DEC: u8 = 2;
    /// Bits after the decimal point
    pub const FRC: u8 = 6;
    /// Literal bits after the decimal point. 2 ^ FRC
    pub const FRCPOW: u8 = 64;
    /// Minimum value in bits
    pub const MINB: u8 = 0x80;
    /// Maximum value in bits
    pub const MAXB: u8 = 0x7F;
    /// Minimum value for an 8-bit Certum.
    /// 
    /// Decimal: -1.984375
    pub const MIN: Self = Self { bits: Self::MINB };
    /// Maximum value for an 8-bit Certum.
    /// 
    /// Decimal: 1.984375
    pub const MAX: Self = Self { bits: Self::MAXB };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = -2.0f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 1.984375f64;
    /// Grain - Smallest possible absolute quantity of this type
    pub const GRN: Self = Self { bits: 0b1 };
    /// One - The certum equivalent of integer 1
    pub const ONE: u8 = 65;

    /// Get a new certum from raw bits
    pub fn of(bits: u8) -> Self {
        Self { bits }
    }

    /// Get the sign bit of the current certum in the proper location
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn sign_inverter(&self) -> u8 {
        if self.bits & Self::MINB == Self::MINB { Self::MINB }
        else { Self::MAXB }
    }

    /// Get the binary sign of the current certum
    /// 
    /// 1 = negative, 0 = zero or positive
    pub fn bin_sign(&self) -> u8 {
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

    /// Extract the bits as its signed counterpart
    pub fn as_signed_bits(&self) -> i8 {
        self.bits as i8
    }

    /// Print line to console with a name and bits
    pub fn log_bits(&self) {
        println!("0b{:08b}", self.bits);
    }

    /// Print line to console with a name and hexadecimal bits
    pub fn log_hex(&self) {
        println!("0x{:02X}", self.bits);
    }

    /// Print line to console with a name and float value
    pub fn log_value(&self) {
        println!("{:.32}", f64::from(self));
    }
}