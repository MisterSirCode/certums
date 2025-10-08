use certums::uc8;
use std::f32;
use std::f64;

fn main() {
    let val = uc8::from(f32::consts::PI);
    println!("{:064b}", f64::consts::PI.to_bits());
    println!("{:.64}", f64::consts::PI);
    println!("{:08b}", val.bits);
    let flt = f32::from(val);
    println!("{:.8}", flt);
}