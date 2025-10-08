use certums::c32;
use std::f32;
use std::f64;

fn main() {
    let val = c32::from(f64::consts::PI);
    println!("{:032b}", f64::consts::PI.to_bits());
    println!("{:.32}", f64::consts::PI);
    println!("{:032b}", val.bits);
    let flt = f64::from(val);
    println!("{:.32}", flt);
}