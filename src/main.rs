use {
    core::{f32, f64},
    certums::{c8, uc8, c16, uc16, c32, uc32, c64, uc64},
    certums::{from_direct, from_right_shift, from_left_shift}
};

fn main() {
    let val8 = uc8::from(f64::consts::E);
    let val16 = uc16::from(f64::consts::E);
    let val32 = uc32::from(f64::consts::E);
    let val64 = uc64::from(f64::consts::E);
    println!("{:08b}", val8.bits);
    println!("{:.8}", f64::from(val8));
    println!("{:016b}", val16.bits);
    println!("{:.16}", f64::from(val16));
    println!("{:032b}", val32.bits);
    println!("{:.32}", f64::from(val32));
    println!("{:064b}", val64.bits);
    println!("{:.64}", f64::from(val64));
}