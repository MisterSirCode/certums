use certums::uc64;
use std::f32;
use std::f64;

fn main() {
    let val = uc64::from(f64::consts::PI);
    println!("{:064b}", val.bits);
    let flt = f64::from(val);
    println!("{:.64}", flt);
}