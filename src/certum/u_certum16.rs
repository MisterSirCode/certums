#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// Define a generic 16-bit Unsigned Certum
pub struct uc16 {
    /// The raw bits of the certum
    /// 
    /// 3 Integer bits, 13 Fraction bits
    pub bits: u16
}

impl uc16 {
    /// Minimum value for a 16-bit Unsigned Certum.
    /// 
    /// Decimal: 0
    pub const MIN: uc16 = uc16 { bits: 0 };
    /// Maximum value for a 16-bit Unsigned Certum.
    /// 
    /// Decimal: 7.9998779296875
    pub const MAX: uc16 = uc16 { bits: 0xFFFF };
    /// Minimum value as a 64-bit Float
    pub const MINF: f64 = 0f64;
    /// Maximum value as a 64-bit Float
    pub const MAXF: f64 = 7.9998779296875f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.1416015625
    pub const PI: uc16 = uc16 { bits: 0x6488 };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.71826171875
    pub const E: uc16 = uc16 { bits: 0x56FC };

    /// Return the binary components of the current certum
    /// 
    /// (Integer Component, Fraction Component)
    pub fn components(&self) -> (u16, u16) {
        // Order ints smallest component as LSB
        // 16 bits - 3 int bits = 13 bit shifts
        let int = self.bits >> 13;
        // Order fraction's largest component as MSB
        // 16 bits - 13 frc bits = 3 bit shifts
        let frc = self.bits << 3;
        (int, frc)
    }

    /// Clamp a u64 and round to a u16 properly.
    /// 
    /// Right-shift MSB to (64 - 17), carry case with + 1, right-shift MSB to make 16 bits. Clamp to u16
    pub fn u64_round(val: u64) -> u16 {
        ((val + 0x800000000000) >> 48) as u16
    }
}