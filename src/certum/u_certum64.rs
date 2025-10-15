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
    pub const MIN: uc64 = uc64 { bits: 0 };
    /// Maximum value for a 64-bit Unsigned Certum.
    /// 
    /// Decimal: 31.99999999999999999826527652402319290558807551860809326171875
    pub const MAX: uc64 = uc64 { bits: 0xFFFFFFFFFFFFFFFF };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = 0f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 32f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.141592653589793115997963468544185161590576171875
    pub const PI: uc64 = uc64 { bits: 0x1921FB54442D1800 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.718281828459045090795598298427648842334747314453125
    pub const E: uc64 = uc64 { bits: 0x15BF0A8B14576900 };

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
}