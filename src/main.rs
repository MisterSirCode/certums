use {
    core::{f32, f64},
    certums::{c8, uc8, c16, uc16, c32, uc32, c64, uc64},
    certums::{from_direct, from_right_shift, from_left_shift}
};

fn main() {
    let value = f64::consts::E;
    let val8 = c8::from(value);
    let val16 = c16::from(value);
    let val32 = c32::from(value);
    let val64 = c64::from(value);
    println!("0b{:08b}", val8.bits);
    println!("{:.8}", f64::from(val8));
    println!("0b{:016b}", val16.bits);
    println!("{:.16}", f64::from(val16));
    println!("0b{:032b}", val32.bits);
    println!("{:.32}", f64::from(val32));
    println!("0b{:064b}", val64.bits);
    println!("{:.64}", f64::from(val64));
    let uval8 = uc8::from(value);
    let uval16 = uc16::from(value);
    let uval32 = uc32::from(value);
    let uval64 = uc64::from(value);
    println!("0b{:08b}", uval8.bits);
    println!("{:.8}", f64::from(uval8));
    println!("0b{:016b}", uval16.bits);
    println!("{:.16}", f64::from(uval16));
    println!("0b{:032b}", uval32.bits);
    println!("{:.32}", f64::from(uval32));
    println!("0b{:064b}", uval64.bits);
    println!("{:.64}", f64::from(uval64));
}