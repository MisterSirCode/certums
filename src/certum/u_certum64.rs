#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// Define a generic 64-bit Unsigned Certum
pub struct uc64 {
    /// The raw bits of the certum
    /// 
    /// 5 Integer bits, 59 Fraction bits
    pub bits: u64
}

impl uc64 {
    /// Minimum value for a 64-bit Unsigned Certum.
    /// 
    /// Decimal: 0
    pub const MIN: Self = Self { bits: 0 };
    /// Maximum value for a 64-bit Unsigned Certum.
    /// 
    /// Decimal: 31.99999999999999999826527652402319290558807551860809326171875
    pub const MAX: Self = Self { bits: 0xFFFFFFFFFFFFFFFF };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = 0f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 32f64;
    /// Grain - Smallest possible absolute quantity of this type
    pub const GRN: Self = Self { bits: 0b1 };
    /// Archimedes' Constant - π
    /// 
    /// Decimal: 3.1415926535897932374286067869206817704252898693084716796875
    /// Exact:   3.14159265358979323
    /// 
    /// Digits of Accuracy: 18
    /// 
    pub const PI: Self = Self { bits: 0x1921FB54442D1846 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.71828182845904523477764680450263767852447926998138427734375
    /// Exact:   2.71828182845904523
    /// 
    /// Digits of Accuracy: 18
    /// 
    pub const E: Self = Self { bits: 0x15BF0A8B14576953 };

    /// Return the binary components of the current certum
    /// 
    /// (Integer Component, Fraction Component)
    pub fn components(&self) -> (u64, u64) {
        // Order ints smallest component as LSB
        // 64 bits - 5 int bits = 59 bit shifts
        let int = self.bits >> 59;
        // Order fraction's largest component as MSB
        // 64 bits - 59 frc bits = 5 bit shifts
        let frc = self.bits << 5;
        (int, frc)
    }

    /// Internal use only
    pub fn u64_round(val: u64) -> u64 {
        val
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