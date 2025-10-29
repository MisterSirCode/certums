#![allow(unused_imports)]

use {
    certums::{c128, c16, c32, c64, c8, from_direct, from_left_shift, from_right_shift, uc128, uc16, uc32, uc64, uc8, u256, utils::display_certums}, core::{f32, f64}
};

fn main() {
    // display_certums(f32::consts::PI);
    // (c64::from(0.5) + c64::from(0.25)).log_value();
    // (c64::from(2) * c64::from(0.5)).log_value();
    // print!("\n");
    // c64::from(2).log_bits();
    // c64::from(0.25).log_bits();
    // (c64::from(16) * c64::from(0.25)).log_value();
    // (c64::from(2) * c64::from(0.25)).log_bits();
    // print!("\n");
    // c64::from(2).log_bits();
    // c64::from(3).log_bits();
    // (c64::from(2.0) * c64::from(3.0)).log_value();
    
    let ut = u256::MAX;
    let ut2 = u256::from(u128::MAX);
    let ut3 = u256::from(100);
    let ut4 = ut2 - ut3;
    println!("{:0128b}{:0128b}\n", ut2.bits.0, ut2.bits.1);
    println!("{:0128b}{:0128b}\n", ut3.bits.0, ut3.bits.1);
    println!("{:0128b}{:0128b}", ut4.bits.0, ut4.bits.1);
}