#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// Define a generic 32-bit Unsigned Certum
pub struct uc32 {
    /// The raw bits of the certum
    /// 
    /// 4 Integer bits, 28 Fraction bits
    pub bits: u32
}

impl uc32 {
    /// Minimum value for a 32-bit Unsigned Certum.
    /// 
    /// Decimal: 0
    pub const MIN: uc32 = uc32 { bits: 0 };
    /// Maximum value for a 32-bit Unsigned Certum.
    /// 
    /// Decimal: 15.9999999962747097015380859375
    pub const MAX: uc32 = uc32 { bits: 0xFFFFFFFF };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = 0f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 15.99999999627471f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.1415926516056060791015625
    /// Exact:   3.14159265
    /// 
    /// Digits of Accuracy: 9
    /// 
    pub const PI: uc32 = uc32 { bits: 0x3243F6A8 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.718281827867031097412109375
    /// Exact:   2.71828182
    /// 
    /// Digits of Accuracy: 9
    /// 
    pub const E: uc32 = uc32 { bits: 0x2B7E1516 };

    /// Return the binary components of the current certum
    /// 
    /// (Integer Component, Fraction Component)
    pub fn components(&self) -> (u32, u32) {
        // Order ints smallest component as LSB
        // 32 bits - 4 int bits = 28 bit shifts
        let int = self.bits >> 28;
        // Order fraction's largest component as MSB
        // 32 bits - 28 frc bits = 4 bit shifts
        let frc = self.bits << 4;
        (int, frc)
    }

    /// Clamp a u64 and round to a u32 properly.
    /// 
    /// Right-shift MSB to (64 - 33), carry case with + 1, right-shift MSB to make 32 bits. Clamp to u32
    pub fn u64_round(val: u64) -> u32 {
        ((val + 0x80000000) >> 32) as u32
    }
}