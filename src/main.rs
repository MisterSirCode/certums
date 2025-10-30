#![allow(unused_imports)]

use {
    certums::{c8, c16, c32, c64, c128, from_direct, from_left_shift, from_right_shift, u256, uc8, uc16, uc32, uc64, uc128, utils::{QuickLog, display_certums}}, core::{f32, f64}
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
    
    // let ut = u256::MAX;
    let ut2 = u256::from(u128::MAX) << 128;
    let ut3 = u256::from(u128::MAX);
    let ut4 = ut3 - ut2;
    let ut5 = ut4 - u256::from(10);
    ut2.log_bits();
    ut3.log_bits();
    ut4.log_bits();
    ut5.log_bits();
}