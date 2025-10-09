use {
    core::{f32, f64},
    certums::{c8, uc8, c16, uc16, c32, uc32, c64, uc64},
    certums::{from_direct, from_right_shift, from_left_shift},
    certums::utils::display_certums
};

fn main() {
    display_certums(f64::consts::PI);
}