use certums::c8;
use certums::c16;
use std::f32;
use std::f64;

fn main() {
    let val = c16::from(f32::consts::PI);
    let flt = f32::from(val);
    println!("{:032b}", f32::consts::PI.to_bits());
    println!("{:.32}", f32::consts::PI);
    println!("{:016b}", val.bits);
    println!("{:.32}", flt);
}