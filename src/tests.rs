use {
    core::{f32, f64},
    super::{c8, uc8, c16, c32, c64}
};


#[test]
/// Run unit tests for all certums, quartas, dimids, and acutes
pub fn unit_tests() {
    let pi32 = f32::consts::PI;
    let pi64 = f64::consts::PI;
    // Signed Certum-8 Unit Tests
    assert_eq!(c8::from(pi32).bits, 0b01001001);
    assert_eq!(c8::from(pi64).bits, 0b01001001);
    assert_eq!(f32::from(c8::from(0b01001001)), 1.140625f32);
    assert_eq!(f64::from(c8::from(0b01001001)), 1.140625f64);
    // Unsigned Certum-8 Unit Tests
    assert_eq!(uc8::from(pi32).bits, 0b11001001);
    assert_eq!(uc8::from(pi64).bits, 0b11001001);
    assert_eq!(f32::from(uc8::from(0b11001001)), 3.140625f32);
    assert_eq!(f64::from(uc8::from(0b11001001)), 3.140625f64);

    // Signed Certum-16 Unit Tests
    assert_eq!(c16::from(pi32).bits, 0b0110010010001000);
    assert_eq!(c16::from(pi64).bits, 0b0110010010001000);
    assert_eq!(f32::from(c16::from(0b0110010010001000)), 3.1416015625f32);
    assert_eq!(f64::from(c16::from(0b0110010010001000)), 3.1416015625f64);
    // Unsigned Certum-16 Unit Tests

    // Signed Certum-32 Unit Tests
    assert_eq!(c32::from(pi32).bits, 0b00110010010000111111011011000000);
    assert_eq!(c32::from(pi64).bits, 0b00110010010000111111011010101001);
    assert_eq!(f32::from(c32::from(0b00110010010000111111011011000000)), 3.1415927410125732421875f32);
    assert_eq!(f64::from(c32::from(0b00110010010000111111011010101001)), 3.1415926553308963775634765625f64);
    // Unsigned Certum-32 Unit Tests

    // Signed Certum-32 Unit Tests
    assert_eq!(c64::from(pi32).bits, 0b0001100100100001111110110110000000000000000000000000000000000000);
    assert_eq!(c64::from(pi64).bits, 0b0001100100100001111110110101010001000100001011010001100000000000);
    assert_eq!(f32::from(c64::from(0b0001100100100001111110110110000000000000000000000000000000000000)), 3.1415927410125732421875f32);
    assert_eq!(f64::from(c64::from(0b0001100100100001111110110101010001000100001011010001100000000000)), 3.141592653589793115997963468544185161590576171875);

}