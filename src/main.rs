#![allow(unused_imports)]

use {
    core::{f32, f64},
    certums::{c8, uc8, c16, uc16, c32, uc32, c64, uc64, c128, uc128},
    certums::{from_direct, from_right_shift, from_left_shift},
    certums::utils::display_certums
};

fn main() {
    // display_certums(f32::consts::PI);
    let n1 = c128::from(0.5);
    let n2 = c128::from(-0.25);
    println!("\n{:0128b}", n1.bits);
    println!("{:0128b}", n2.bits);
    println!("{:0128b}", (n1 + n2).bits);
    println!("\n{:}", f64::from(n1));
    println!("{:}", f64::from(n2));
    println!("{:}", f64::from(n1 + n2));
    // println!("{:0128b}", (c128::from(0.5) + c128::from(-0.25)).bits);
    // println!("{:0128b}", (c128::from(0.25)).bits);
    // println!("{:0}", f64::from(c128::from(0.5) + c128::from(-0.25)));
    // println!("{:0}", f64::from(c128::from(0.25)));
}