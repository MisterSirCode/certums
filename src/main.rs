#![allow(unused_imports)]

use {
    certums::{c128, c16, c32, c64, c8, from_direct, from_left_shift, from_right_shift, uc128, uc16, uc32, uc64, uc8, utils::display_certums}, core::{f32, f64}
};

fn main() {
    display_certums(f32::consts::PI);
}