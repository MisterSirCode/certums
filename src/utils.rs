// https://docs.rs/num-traits/latest/src/num_traits/float.rs.html#2049

pub fn integer_decode_f32(f: f32) -> (u64, i16, i8) {
    let bits: u32 = f.to_bits();
    let sign: i8 = if bits >> 31 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 23) & 0xff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0x7fffff) << 1
    } else {
        (bits & 0x7fffff) | 0x800000
    };
    // Exponent bias + mantissa shift
    exponent -= 127 + 23;
    (mantissa as u64, exponent, sign)
}

pub fn integer_decode_f64(f: f64) -> (u64, i16, i8) {
    let bits: u64 = f.to_bits();
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    // Exponent bias + mantissa shift
    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

// Thanks to @kpreid on Rust Discord for these float tidbits. Adjusted slightly to handle negative floats

/// Split a Float32 into its Integer and Fractional components as binary values
pub fn f32_split(mut float: f32) -> (u8, u32, u32) {
    let sign;
    if float < 0.0 { 
        sign = 1u8;
        float *= -1f32;
    } else { sign = 0u8; }
    let integer_part = float.trunc() as u32;
    let fractional_part = ((float - (integer_part as f32)) * 2f32.powi(32)) as u32;
    (sign, integer_part, fractional_part)
}

/// Split a Float64 into its Integer and Fractional Components as binary values
pub fn f64_split(mut float: f64) -> (u8, u64, u64) {
    let sign;
    if float < 0.0 { 
        sign = 1u8;
        float *= -1f64;
    } else { sign = 0u8; }
    let integer_part = float.trunc() as u64;
    let fractional_part = ((float - (integer_part as f64)) * 2f64.powi(64)) as u64;
    (sign, integer_part, fractional_part)
}

/// Clamp a u16 and round to a u8 properly.
/// 
/// Right-shift MSB to (16 - 9), carry case with + 1, right-shift MSB to make 8 bits. Clamp to u8
pub fn u16_to_u8_round(val: u16) -> u8 {
    ((val + 0x80) >> 8) as u8
}

/// Clamp a u32 and round to a u8 properly.
/// 
/// Right-shift MSB to (32 - 9), carry case with + 1, right-shift MSB to make 8 bits. Clamp to u8
pub fn u32_to_u8_round(val: u32) -> u8 {
    ((val + 0x800000) >> 24) as u8
}

/// Clamp a u64 and round to a u8 properly.
/// 
/// Right-shift MSB to (64 - 9), carry case with + 1, right-shift MSB to make 8 bits. Clamp to u8
pub fn u64_to_u8_round(val: u64) -> u8 {
    ((val + 0x80000000000000) >> 56) as u8
}

/// Clamp a u32 and round to a u16 properly.
/// 
/// Right-shift MSB to (32 - 17), carry case with + 1, right-shift MSB to make 16 bits. Clamp to u16
pub fn u32_to_u16_round(val: u32) -> u16 {
    ((val + 0x8000) >> 16) as u16
}

/// Clamp a u64 and round to a u16 properly.
/// 
/// Right-shift MSB to (64 - 17), carry case with + 1, right-shift MSB to make 16 bits. Clamp to u16
pub fn u64_to_u16_round(val: u64) -> u16 {
    ((val + 0x800000000000) >> 48) as u16
}

/// Clamp a u64 and round to a u32 properly.
/// 
/// Right-shift MSB to (64 - 33), carry case with + 1, right-shift MSB to make 32 bits. Clamp to u32
pub fn u64_to_u32_round(val: u64) -> u32 {
    ((val + 0x80000000) >> 32) as u32
}

use {
    core::{f32, f64},
    super::{c8, uc8, c16, uc16, c32, uc32, c64, uc64}
};

pub fn display_certums(value: f64) {
    let val8 = c8::from(value);
    let val16 = c16::from(value);
    let val32 = c32::from(value);
    let val64 = c64::from(value);
    println!("\nSigned Types: ");
    println!("0x{:02X}", val8.bits);
    println!("{:.8}", f64::from(val8));
    println!("0x{:04X}", val16.bits);
    println!("{:.16}", f64::from(val16));
    println!("0x{:08X}", val32.bits);
    println!("{:.32}", f64::from(val32));
    println!("0x{:016X}", val64.bits);
    println!("{:.64}", f64::from(val64));
    let uval8 = uc8::from(value);
    let uval16 = uc16::from(value);
    let uval32 = uc32::from(value);
    let uval64 = uc64::from(value);
    println!("\nUnsigned Types: ");
    println!("0x{:02X}", uval8.bits);
    println!("{:.8}", f64::from(uval8));
    println!("0x{:04X}", uval16.bits);
    println!("{:.16}", f64::from(uval16));
    println!("0x{:08X}", uval32.bits);
    println!("{:.32}", f64::from(uval32));
    println!("0x{:016X}", uval64.bits);
    println!("{:.64}", f64::from(uval64));
}