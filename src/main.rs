#![allow(unused_imports)]

use {
    core::{f32, f64},
    certums::{c8, uc8, c16, uc16, c32, uc32, c64, uc64},
    certums::{from_direct, from_right_shift, from_left_shift},
    certums::utils::display_certums
};

fn main() {
    display_certums(f64::consts::PI);

    let n1 = c16::from(f64::consts::PI);
    let n2 = -c16::from(1f64);
    println!("\n{:.016}", f64::from(n1 + n2));
    println!("\n{:.016},            {:.016}", c16::from(-1f64).bits, (-c16::from(1f64)).bits);
    println!("{:016b}, {:016b}", c16::from(-1f64).bits, (-c16::from(1f64)).bits);
}