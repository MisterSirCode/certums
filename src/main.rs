use certums::c8;
use std::f32;
use std::f64;

fn main() {
    let val = c8::from(f32::consts::PI);
    let flt = f64::from(val);
    println!("{:}", f32::consts::PI);
    println!("{:08b}", val.bits);
    println!("{:}", flt);
}