#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
/// Define a generic 128-bit Unsigned Certum
pub struct uc128 {
    /// The raw bits of the certum
    /// 
    /// 6 Integer bits, 122 Fraction bits
    pub bits: u128
}

impl uc128 {
    /// Minimum value for a 128-bit Unsigned Certum.
    /// 
    /// Decimal: 0
    pub const MIN: uc128 = uc128 { bits: 0 };
    /// Maximum value for a 128-bit Unsigned Certum.
    /// 
    /// Decimal: 63.99999999999999999999999999999999999981192090386843399872500215404444069154901351091646599655859972699545323848724365234375
    pub const MAX: uc128 = uc128 { bits: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF };
    /// Minimum value as a 128-bit Float
    pub const MINF: f64 = 0f64;
    /// Maximum value as a 128-bit Float
    pub const MAXF: f64 = 64f64;
    /// Archimede's Constant - π
    /// 
    /// Decimal: 3.14159265358979323846264338327950288418353141478922216077851131738713791456774700357190699406828571227379143238067626953125
    /// Exact:   3.1415926535897932384626433832795028841
    /// 
    /// Digits of Accuracy: 38
    /// 
    pub const PI: uc128 = uc128 { bits: 0xC90FDAA22168C234C4C6628B80DC1CD };
    /// Eulers's Number - e
    /// 
    /// Decimal: 2.71828182845904523536028747135266249773554059547040010579213066699799290773116567342115867944585261284373700618743896484375
    /// Exact:   2.7182818284590452353602874713526624977
    /// 
    /// Digits of Accuracy: 38
    /// 
    pub const E: uc128 = uc128 { bits: 0xADF85458A2BB4A9AAFDC5620273D3CF };

    /// Return the binary components of the current certum
    /// 
    /// (Integer Component, Fraction Component)
    pub fn components(&self) -> (u128, u128) {
        // Order ints smallest component as LSB
        // 128 bits - 6 int bits = 122 bit shifts
        let int = self.bits >> 122;
        // Order fraction's largest component as MSB
        // 128 bits - 122 frc bits = 6 bit shifts
        let frc = self.bits << 6;
        (int, frc)
    }

    /// Internal use only
    pub fn u64_round(val: u128) -> u128 {
        val
    }
}