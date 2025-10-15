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
    assert_eq!(c16::from(pi32), c16::PI);
    assert_eq!(c16::from(pi64), c16::PI);
    assert_eq!(f32::from(c16::PI), 3.1416015625);
    assert_eq!(f64::from(c16::PI), 3.1416015625);

    // Unsigned Certum-16 Unit Tests
    assert_eq!(uc16::from(pi32), uc16::PI);
    assert_eq!(uc16::from(pi64), uc16::PI);
    assert_eq!(f32::from(uc16::PI), 3.1416015625);
    assert_eq!(f64::from(uc16::PI), 3.1416015625);

    // Signed Certum-32 Unit Tests
    assert_eq!(c32::from(pi32).bits, 0x3243F6C0);
    assert_eq!(c32::from(pi64),      c32::PI);
    assert_eq!(f32::from(c32::from(0x3243F6C0)), 3.1415927410125732421875);
    assert_eq!(f64::from(c32::PI),               3.1415926553308963775634765625);

    // Unsigned Certum-32 Unit Tests
    assert_eq!(uc32::from(pi32).bits, 0x3243F6C0);
    assert_eq!(uc32::from(pi64),      uc32::PI);
    assert_eq!(f32::from(uc32::from(0x3243F6C0)), 3.1415927410125732421875);
    assert_eq!(f64::from(uc32::PI),               3.1415926553308963775634765625);

    // Signed Certum-64 Unit Tests
    assert_eq!(c64::from(pi32).bits, 0x1921FB6000000000);
    assert_eq!(c64::from(pi64),      c64::PI);
    assert_eq!(f32::from(c64::from(0x1921FB6000000000)), 3.1415927410125732421875);
    assert_eq!(f64::from(c64::PI),                       3.141592653589793115997963468544185161590576171875);

    // Unsigned Certum-64 Unit Tests
    assert_eq!(uc64::from(pi32).bits, 0x1921FB6000000000);
    assert_eq!(uc64::from(pi64),      uc64::PI);
    assert_eq!(f32::from(uc64::from(0x1921FB6000000000)), 3.1415927410125732421875);
    assert_eq!(f64::from(uc64::PI),                       3.141592653589793115997963468544185161590576171875);

    // Signed Certum-128 Unit Tests
    assert_eq!(c128::from(pi32).bits, 0x1921FB60000000000000000000000000);
    assert_eq!(c128::from(pi64),      c128::PI);
    assert_eq!(f32::from(c128::from(0x1921FB60000000000000000000000000)), 3.1415927410125732421875);
    assert_eq!(f64::from(c128::PI),                                       3.141592653589793115997963468544185161590576171875);

    // Unsigned Certum-128 Unit Tests
    assert_eq!(uc128::from(pi32).bits, 0x0C90FDB0000000000000000000000000);
    assert_eq!(uc128::from(pi64),      uc128::PI);
    assert_eq!(f32::from(uc128::from(0x0C90FDB0000000000000000000000000)), 3.1415927410125732421875);
    assert_eq!(f64::from(uc128::PI),                                       3.141592653589793115997963468544185161590576171875);
}

#[test]
// Run all unit tests for addition of types
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
    assert_eq!(c128::from(0.5) + c128::from(-0.25), c128::from(0.25));
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