use certums::c8;
use std::f32;

fn main() {
    let val = c8::from(f32::consts::PI);
    println!("{:08b}", val.bits);
}