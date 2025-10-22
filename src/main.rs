#![allow(unused_imports)]

use {
    certums::{c128, c16, c32, c64, c8, from_direct, from_left_shift, from_right_shift, uc128, uc16, uc32, uc64, uc8, utils::display_certums}, core::{f32, f64}
};

pub fn conversion_tests() {
    assert_eq!(c8::from(c8::MAX).bits, 0x7F);
    assert_eq!(c16::from(c8::MAX).bits, 0x3F80);
    assert_eq!(c32::from(c8::MAX).bits, 0x1FC00000);
    assert_eq!(c64::from(c8::MAX).bits, 0xFE0000000000000);
    assert_eq!(c128::from(c8::MAX).bits, 0x7F00000000000000000000000000000);
    assert_eq!(c8::from(c16::MAX).bits, 0x7F);
    assert_eq!(c16::from(c16::MAX).bits, 0x7FFF);
    assert_eq!(c32::from(c16::MAX).bits, 0x3FFF8000);
    assert_eq!(c64::from(c16::MAX).bits, 0x1FFFC00000000000);
    assert_eq!(c128::from(c16::MAX).bits, 0x0FFFE000000000000000000000000000);
    assert_eq!(c8::from(c16::MAX).bits, 0x7F);
    assert_eq!(c16::from(c16::MAX).bits, 0x7FFF);
    assert_eq!(c32::from(c16::MAX).bits, 0x3FFF8000);
    assert_eq!(c64::from(c16::MAX).bits, 0x1FFFC00000000000);
    assert_eq!(c128::from(c16::MAX).bits, 0x0FFFE000000000000000000000000000);

}

fn main() {
    // display_certums(f64::consts::PI);
    // println!("\n{:064b}", c64::from(f64::NEG_INFINITY).bits);
    // let n1 = c128::from(30f64);
    // let n2 = c128::from(-31f64);
    // println!("\n{:0128b}", n1.bits);
    // println!("{:0128b}", n2.bits);
    // println!("{:0128b}", (n1 + n2).bits);
    // println!("\n{:}", f64::from(n1));
    // println!("{:}", f64::from(n2));
    // println!("{:}", f64::from(n1 + n2));

    // conversion_tests();

    println!("0b{:032b}", c32::MAX.bits);
    println!("0b{:016b}", c16::from(c32::MAX).bits);
    println!("0b{:032b}", c32::from(c32::MAX).bits);
    println!("0b{:064b}", c64::from(c32::MAX).bits);
    println!("0b{:0128b}", c128::from(c32::MAX).bits);
    // println!("{:032b}", c32::PI.bits);
    // println!("{:016b}", c16::from(c32::PI).bits);
    // assert_eq!(f64::from(c32::from(c8::MAX)), c8::MAXF);
    // assert_eq!(f64::from(c64::from(c8::MAX)), c8::MAXF);
    // assert_eq!(f64::from(c128::from(c8::MAX)), c8::MAXF);
    // println!("{:0128b}", (c128::from(0.5) + c128::from(-0.25)).bits);
    // println!("{:0128b}", (c128::from(0.25)).bits);
    // println!("{:0}", f64::from(c128::from(0.5) + c128::from(-0.25)));
    // println!("{:0}", f64::from(c128::from(0.25)));
}