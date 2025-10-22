#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// Define a generic 8-bit Unsigned Certum
pub struct uc8 {
    /// The raw bits of the certum
    /// 
    /// 2 Integer bits, 6 Fraction bits
    pub bits: u8
}

impl uc8 {
    /// Minimum value for an 8-bit Unsigned Certum.
    /// 
    /// Decimal: 0
    pub const MIN: uc8 = uc8 { bits: 0 };
    /// Maximum value for an 8-bit Unsigned Certum.
    /// 
    /// Decimal: 3.984375
    pub const MAX: uc8 = uc8 { bits: 0xFF };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = 0f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 3.984375f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.140625
    /// Exact:   3.14
    /// 
    /// Digits of Accuracy: 3
    /// 
    pub const PI: uc8 = uc8 { bits: 0xC9 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.71875
    /// Exact:   2.718
    /// 
    /// Digits of Accuracy: 4
    /// 
    pub const E: uc8 = uc8 { bits: 0xAE };

    /// Return the binary components of the current certum
    /// 
    /// (Integer Component, Fraction Component)
    pub fn components(&self) -> (u8, u8) {
        // Order ints smallest component as LSB
        // 8 bits - 2 int bits = 6 bit shifts
        let int = self.bits >> 6;
        // Order fraction's largest component as MSB
        // 8 bits - 6 frc bits = 2 bit shifts
        let frc = self.bits << 2;
        (int, frc)
    }

    /// Clamp a u64 and round to a u8 properly.
    /// 
    /// Right-shift MSB to (64 - 9), carry case with + 1, right-shift MSB to make 8 bits. Clamp to u8
    pub fn u64_round(val: u64) -> u8 {
        ((val + 0x80000000000000) >> 56) as u8
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
        println!("0b{:.32}", f64::from(self));
    }
}