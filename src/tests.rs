#![allow(unused_imports)]

use {
    core::{f32, f64},
    super::{c8, uc8, c16, uc16, c32, uc32, c64, uc64, c128, uc128},
    crate::{from_direct, from_right_shift, from_left_shift}
};

#[test]
/// Run unit tests for interpreting all certums, quartas, dimids, and acutes
pub fn interpretation_tests() {
    let pi32 = f32::consts::PI;
    let pi64 = f64::consts::PI;

    // Signed Certum-8 Unit Tests
    assert_eq!(c8::from(pi32), c8::MAX);
    assert_eq!(c8::from(pi64), c8::MAX);
    assert_eq!(f32::from(c8::MAX), c8::MAXF as f32);
    assert_eq!(f64::from(c8::MAX), c8::MAXF);

    // Unsigned Certum-8 Unit Tests
    assert_eq!(uc8::from(pi32), uc8::PI);
    assert_eq!(uc8::from(pi64), uc8::PI);
    assert_eq!(f32::from(uc8::PI), 3.140625);
    assert_eq!(f64::from(uc8::PI), 3.140625);

    // Signed Certum-16 Unit Tests
    assert_eq!(c16::from(pi32).bits, 0x6488);
    assert_eq!(c16::from(pi64).bits, 0x6488);
    assert_eq!(f32::from(c16::PI), 3.1414794921875);
    assert_eq!(f64::from(c16::PI), 3.1414794921875);

    // Unsigned Certum-16 Unit Tests
    assert_eq!(uc16::from(pi32).bits, 0x6488);
    assert_eq!(uc16::from(pi64).bits, 0x6488);
    assert_eq!(f32::from(uc16::PI), 3.1414794921875);
    assert_eq!(f64::from(uc16::PI), 3.1414794921875);

    // Signed Certum-32 Unit Tests
    assert_eq!(c32::from(pi32).bits, 0x3243F6C0);
    assert_eq!(c32::from(pi64).bits, 0x3243F6A9);
    assert_eq!(f32::from(c32::from(0x3243F6C0)), 3.1415927410125732421875);
    assert_eq!(f64::from(c32::PI),               3.141592651605606);

    // Unsigned Certum-32 Unit Tests
    assert_eq!(uc32::from(pi32).bits, 0x3243F6C0);
    assert_eq!(uc32::from(pi64).bits, 0x3243F6A9);
    assert_eq!(f32::from(uc32::from(0x3243F6C0)), 3.1415927410125732421875);
    assert_eq!(f64::from(uc32::PI),               3.141592651605606);

    // Signed Certum-64 Unit Tests
    assert_eq!(c64::from(pi32).bits, 0x1921FB6000000000);
    assert_eq!(c64::from(pi64).bits, 0x1921FB54442D1800);
    assert_eq!(f32::from(c64::from(0x1921FB6000000000)), 3.1415927410125732421875);
    assert_eq!(f64::from(c64::PI),                       3.141592653589793);

    // Unsigned Certum-64 Unit Tests
    assert_eq!(uc64::from(pi32).bits, 0x1921FB6000000000);
    assert_eq!(uc64::from(pi64).bits, 0x1921FB54442D1800);
    assert_eq!(f32::from(uc64::from(0x1921FB6000000000)), 3.1415927410125732421875);
    assert_eq!(f64::from(uc64::PI),                       3.141592653589793);

    // Signed Certum-128 Unit Tests
    assert_eq!(c128::from(pi32).bits, 0x0C90FDB0000000000000000000000000);
    assert_eq!(c128::from(pi64).bits, 0xC90FDAA22168C000000000000000000);
    assert_eq!(f32::from(c128::from(0x0C90FDB0000000000000000000000000)), 3.1415927410125732421875);
    assert_eq!(f64::from(c128::PI),                                       3.141592653589793);

    // Unsigned Certum-128 Unit Tests
    assert_eq!(uc128::from(pi32).bits, 0x0C90FDB0000000000000000000000000);
    assert_eq!(uc128::from(pi64).bits, 0xC90FDAA22168C000000000000000000);
    assert_eq!(f32::from(uc128::from(0x0C90FDB0000000000000000000000000)), 3.1415927410125732421875);
    assert_eq!(f64::from(uc128::PI),                                       3.141592653589793);
}

#[test]
/// Conversion Tests
pub fn conversion_tests() {
    // Check if type casting works as expected
    let min_low = c8::MIN;
    let min_high = c128::MAX;
    let max_low = c8::MIN;
    let max_high = c128::MAX;
    assert_eq!(c8::from(min_low),    c8::MIN);
    assert_eq!(c16::from(min_low),   c16::MIN);
    assert_eq!(c32::from(min_low),   c32::MIN);
    assert_eq!(c64::from(min_low),   c64::MIN);
    assert_eq!(c128::from(min_low),  c128::MIN);
    assert_eq!(c8::from(min_high),   c8::MAX);
    assert_eq!(c16::from(min_high),  c16::MAX);
    assert_eq!(c32::from(min_high),  c32::MAX);
    assert_eq!(c64::from(min_high),  c64::MAX);
    assert_eq!(c128::from(min_high), c128::MAX);
    assert_eq!(c8::from(max_low),    c8::MIN);
    assert_eq!(c16::from(max_low),   c16::MIN);
    assert_eq!(c32::from(max_low),   c32::MIN);
    assert_eq!(c64::from(max_low),   c64::MIN);
    assert_eq!(c128::from(max_low),  c128::MIN);
    assert_eq!(c8::from(max_high),   c8::MAX);
    assert_eq!(c16::from(max_high),  c16::MAX);
    assert_eq!(c32::from(max_high),  c32::MAX);
    assert_eq!(c64::from(max_high),  c64::MAX);
    assert_eq!(c128::from(max_high), c128::MAX);
}

#[test]
/// Constants Tests
pub fn constants_tests() {
    assert_eq!(f64::from(c8::MIN),   c8::MINF);
    assert_eq!(f64::from(c8::MAX),   c8::MAXF);
    assert_eq!(f64::from(uc8::MIN),  uc8::MINF);
    assert_eq!(f64::from(uc8::MAX),  uc8::MAXF);
    assert_eq!(f64::from(c16::MIN),  c16::MINF);
    assert_eq!(f64::from(c16::MAX),  c16::MAXF);
    assert_eq!(f64::from(uc16::MIN), uc16::MINF);
    assert_eq!(f64::from(uc16::MAX), uc16::MAXF);
    assert_eq!(f64::from(c32::MIN),  c32::MINF);
    assert_eq!(f64::from(c32::MAX),  c32::MAXF);
    assert_eq!(f64::from(uc32::MIN), uc32::MINF);
    assert_eq!(f64::from(uc32::MAX), uc32::MAXF);
    assert_eq!(f64::from(c64::MIN),  c64::MINF);
    assert_eq!(f64::from(c64::MAX),  c64::MAXF);
    assert_eq!(f64::from(uc64::MIN), uc64::MINF);
    assert_eq!(f64::from(uc64::MAX), uc64::MAXF);
    assert_eq!(f64::from(c128::MIN), c64::MINF);
    assert_eq!(f64::from(c128::MAX), c64::MAXF);
    assert_eq!(f64::from(uc128::MIN), uc64::MINF);
    assert_eq!(f64::from(uc128::MAX), uc64::MAXF);
    assert_eq!(f64::from(uc8::PI),   3.140625);
    assert_eq!(f64::from(c16::PI),   3.1414794921875);
    assert_eq!(f64::from(uc16::PI),  3.1414794921875);
    assert_eq!(f64::from(c32::PI),   3.141592651605606);
    assert_eq!(f64::from(uc32::PI),  3.141592651605606);
    assert_eq!(f64::from(c64::PI),   3.141592653589793);
    assert_eq!(f64::from(uc64::PI),  3.141592653589793);
    assert_eq!(f64::from(c128::PI),  3.141592653589793);
    assert_eq!(f64::from(uc128::PI), 3.141592653589793);
    assert_eq!(f64::from(uc8::E),   2.71875);
    assert_eq!(f64::from(c16::E),   2.71826171875);
    assert_eq!(f64::from(uc16::E),  2.71826171875);
    assert_eq!(f64::from(c32::E),   2.718281827867031);
    assert_eq!(f64::from(uc32::E),  2.718281827867031);
    assert_eq!(f64::from(c64::E),   2.718281828459045);
    assert_eq!(f64::from(uc64::E),  2.718281828459045);
    assert_eq!(f64::from(c128::E),  2.718281828459045);
    assert_eq!(f64::from(uc128::E), 2.718281828459045);
}


#[test]
/// Run all unit tests for addition of types
pub fn addition_tests() {
    // upper + -lower
    // upper + lower
    // lower + -upper
    // lower + upper
    assert_eq!(c8::from(0.5) + c8::from(-0.25), c8::from(0.25));
    assert_eq!(c8::from(0.5) + c8::from(0.25), c8::from(0.75));
    assert_eq!(c8::from(0.25) + c8::from(-0.5), c8::from(-0.25));
    assert_eq!(c8::from(0.25) + c8::from(0.5), c8::from(0.75));
    assert_eq!(c16::from(0.5) + c16::from(-0.25), c16::from(0.25));
    assert_eq!(c16::from(0.5) + c16::from(0.25), c16::from(0.75));
    assert_eq!(c16::from(0.25) + c16::from(-0.5), c16::from(-0.25));
    assert_eq!(c16::from(0.25) + c16::from(0.5), c16::from(0.75));
    assert_eq!(c32::from(0.5) + c32::from(-0.25), c32::from(0.25));
    assert_eq!(c32::from(0.5) + c32::from(0.25), c32::from(0.75));
    assert_eq!(c32::from(0.25) + c32::from(-0.5), c32::from(-0.25));
    assert_eq!(c32::from(0.25) + c32::from(0.5), c32::from(0.75));
    assert_eq!(c64::from(0.5) + c64::from(-0.25), c64::from(0.25));
    assert_eq!(c64::from(0.5) + c64::from(0.25), c64::from(0.75));
    assert_eq!(c64::from(0.25) + c64::from(-0.5), c64::from(-0.25));
    assert_eq!(c64::from(0.25) + c64::from(0.5), c64::from(0.75));
    assert_eq!((c128::from(0.5) + c128::from(-0.25)).bits, c128::from(0.25).bits);
    assert_eq!(c128::from(0.5) + c128::from(0.25), c128::from(0.75));
    assert_eq!(c128::from(0.25) + c128::from(-0.5), c128::from(-0.25));
    assert_eq!(c128::from(0.25) + c128::from(0.5), c128::from(0.75));
}

#[test]
// Run all unit tests for subtraction of types
pub fn subtraction_tests() {
    // upper - -lower
    // upper - lower
    // lower - -upper
    // lower - upper
    assert_eq!(c8::from(0.5) - c8::from(-0.25), c8::from(0.75));
    assert_eq!(c8::from(0.5) - c8::from(0.25), c8::from(0.25));
    assert_eq!(c8::from(0.25) - c8::from(-0.5), c8::from(0.75));
    assert_eq!(c8::from(0.25) - c8::from(0.5), c8::from(-0.25));
    assert_eq!(c16::from(0.5) - c16::from(-0.25), c16::from(0.75));
    assert_eq!(c16::from(0.5) - c16::from(0.25), c16::from(0.25));
    assert_eq!(c16::from(0.25) - c16::from(-0.5), c16::from(0.75));
    assert_eq!(c16::from(0.25) - c16::from(0.5), c16::from(-0.25));
    assert_eq!(c32::from(0.5) - c32::from(-0.25), c32::from(0.75));
    assert_eq!(c32::from(0.5) - c32::from(0.25), c32::from(0.25));
    assert_eq!(c32::from(0.25) - c32::from(-0.5), c32::from(0.75));
    assert_eq!(c32::from(0.25) - c32::from(0.5), c32::from(-0.25));
    assert_eq!(c64::from(0.5) - c64::from(-0.25), c64::from(0.75));
    assert_eq!(c64::from(0.5) - c64::from(0.25), c64::from(0.25));
    assert_eq!(c64::from(0.25) - c64::from(-0.5), c64::from(0.75));
    assert_eq!(c64::from(0.25) - c64::from(0.5), c64::from(-0.25));
    assert_eq!(c128::from(0.5) - c128::from(-0.25), c128::from(0.75));
    assert_eq!(c128::from(0.5) - c128::from(0.25), c128::from(0.25));
    assert_eq!(c128::from(0.25) - c128::from(-0.5), c128::from(0.75));
    assert_eq!(c128::from(0.25) - c128::from(0.5), c128::from(-0.25));
}