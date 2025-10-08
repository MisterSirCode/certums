#[cfg(test)]
use core::f64;
#[cfg(test)]
use core::f32;
#[cfg(test)]
use super::c8;
#[cfg(test)]
use super::c16;
#[cfg(test)]
use super::c32;


#[test]
/// Run test cases of every major variation of every posit conversion type
/// Default ES values are acquired from `log(bits)/log(2) - 2`, which is
///  (most likely) the most precise ES value before sacrificing fractional precision for whole precision.
///  Use higher/lower ES values to optimize for whole number precision.
///  (This will vary a lot in practicality for smaller posits)
pub fn unit_tests() {
    // Signed Certum-8 Unit Tests
    assert_eq!(c8::from(f32::consts::PI).bits, 0b01001001);
    assert_eq!(c8::from(f64::consts::PI).bits, 0b01001001);
    assert_eq!(f32::from(c8::from(0b01001001)), 1.140625f32);
    assert_eq!(f64::from(c8::from(0b01001001)), 1.140625f64);
    // Unsigned Certum-8 Unit Tests

    // Signed Certum-16 Unit Tests
    assert_eq!(c16::from(f32::consts::PI).bits, 0b0110010010001000);
    assert_eq!(c16::from(f64::consts::PI).bits, 0b0110010010001000);
    assert_eq!(f32::from(c16::from(0b0110010010001000)), 3.1416015625f32);
    assert_eq!(f64::from(c16::from(0b0110010010001000)), 3.1416015625f64);
    // Unsigned Certum-16 Unit Tests

    // Signed Certum-32 Unit Tests
    assert_eq!(c32::from(f32::consts::PI).bits, 0b00110010010000111111011011000000);
    assert_eq!(c32::from(f64::consts::PI).bits, 0b00110010010000111111011010101001);
    assert_eq!(f32::from(c32::from(0b00110010010000111111011011000000)), 3.1415927410125732421875f32);
    assert_eq!(f64::from(c32::from(0b00110010010000111111011010101001)), 3.1415926553308963775634765625f64);

}

// #[test]
// /// Test cases for basic algebra with posits
// pub fn algebra_tests() {
//     println!()
// }

#[test]
pub fn test() {
}