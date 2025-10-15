#![allow(unused_imports)]

use {
    certums::{c128, c16, c32, c64, c8, from_direct, from_left_shift, from_right_shift, uc128, uc16, uc32, uc64, uc8, utils::display_certums}, core::{f32, f64}
};

fn main() {
    display_certums(f64::consts::PI);
    println!("\n{:064b}", c64::from(f64::NEG_INFINITY).bits);
    // let n1 = c128::from(0.5);
    // let n2 = c128::from(-0.25);
    // println!("\n{:0128b}", n1.bits);
    // println!("{:0128b}", n2.bits);
    // println!("{:0128b}", (n1 + n2).bits);
    // println!("\n{:}", f64::from(n1));
    // println!("{:}", f64::from(n2));
    // println!("{:}", f64::from(n1 + n2));
    // println!("{:0128b}", (c128::from(0.5) + c128::from(-0.25)).bits);
    // println!("{:0128b}", (c128::from(0.25)).bits);
    // println!("{:0}", f64::from(c128::from(0.5) + c128::from(-0.25)));
    // println!("{:0}", f64::from(c128::from(0.25)));
}