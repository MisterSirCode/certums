// https://docs.rs/num-traits/latest/src/num_traits/float.rs.html#2049

fn integer_decode_f32(f: f32) -> (u64, i16, i8) {
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

// Thanks to @kpreid on Rust Discord for these tidbits:

/// Split a Float32 into its Integer and Fractional components as binary values
pub fn f32_split(float: f32) -> (u32, u32) {
    assert!(float >= 0.0);
    let integer_part = float.trunc() as u32;
    let fractional_part = ((float - (integer_part as f32)) * 2f32.powi(32)) as u32;
    (integer_part, fractional_part)
}

/// Split a Float64 into its Integer and Fractional Components as binary values
pub fn f64_split(float: f64) -> (u64, u64) {
    assert!(float >= 0.0);
    let integer_part = float.trunc() as u64;
    let fractional_part = ((float - (integer_part as f64)) * 2f64.powi(64)) as u64;
    (integer_part, fractional_part)
}