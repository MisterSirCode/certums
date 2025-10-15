// Signed Certums
pub mod certum8;
pub mod certum16;
pub mod certum32;
pub mod certum64;
pub mod certum128;
// Unsigned Certums
pub mod u_certum8;
pub mod u_certum16;
pub mod u_certum32;
pub mod u_certum64;
pub mod u_certum128;

use {
    std::cmp::{Eq},
    std::ops::{Add, Sub, Neg},
    crate::{
        negate, 
        from_direct, 
        from_right_shift, 
        from_left_shift, 
        equivalent_solo, 
        add_same, 
        sub_same,
        float_casts,
        float_convert_sc, 
        float_convert_uc,
        lossy_float_convert
    },
    super::{c8, uc8, c16, uc16, c32, uc32, c64, uc64, c128, uc128},
    super::utils::{f64_split}
};

// Float Casting

float_convert_sc!(c8,   u8,  8,   2, 0x7F);
float_convert_sc!(c16,  u16, 16,  3, 0x7FFF);
float_convert_sc!(c32,  u32, 32,  4, 0x7FFFFFFF);
float_convert_sc!(c64,  u64, 64,  5, 0x7FFFFFFFFFFFFFFF);
lossy_float_convert!(c128, c64);
float_convert_uc!(uc8,  u8,  8,  2, 0x7F);
float_convert_uc!(uc16, u16, 16, 3, 0x7FFF);
float_convert_uc!(uc32, u32, 32, 4, 0x7FFFFFFF);
float_convert_uc!(uc64, u64, 64, 5, 0x7FFFFFFFFFFFFFFF);
float_casts!(c8,   u8);
float_casts!(c16,  u16);
float_casts!(c32,  u32);
float_casts!(c64,  u64);
float_casts!(c128, u128);
float_casts!(uc8,  u8);
float_casts!(uc16, u16);
float_casts!(uc32, u32);
float_casts!(uc64, u64);

// Negation

negate!(c8);
negate!(c16);
negate!(c32);
negate!(c64);
negate!(c128);
negate!(uc8);
negate!(uc16);
negate!(uc32);
negate!(uc64);
negate!(uc128);

// Type Conversion

from_direct!(c8,   uc8);
from_direct!(c16,  uc16);
from_direct!(c32,  uc32);
from_direct!(c64,  uc64);
from_direct!(c128,  uc128);
from_direct!(uc8,  c8);
from_direct!(uc16, c16);
from_direct!(uc32, c32);
from_direct!(uc64, c64);
from_direct!(uc128, c128);

from_right_shift!(c8,  c16,  u16,  1);
from_right_shift!(c8,  c32,  u32,  2);
from_right_shift!(c8,  c64,  u64,  3);
from_right_shift!(c8,  c128, u128, 4);
from_right_shift!(c16, c32,  u32,  1);
from_right_shift!(c16, c64,  u64,  2);
from_right_shift!(c16, c128, u128, 3);
from_right_shift!(c32, c64,  u64,  1);
from_right_shift!(c32, c128, u128, 1);
from_right_shift!(c64, c128, u128, 1);

from_left_shift!(c128, c64, u64, 1);
from_left_shift!(c128, c32, u32, 2);
from_left_shift!(c128, c16, u16, 3);
from_left_shift!(c128, c8,  u8,  4);
from_left_shift!(c64,  c32, u32, 1);
from_left_shift!(c64,  c16, u16, 2);
from_left_shift!(c64,  c8,  u8,  3);
from_left_shift!(c32,  c16, u16, 1);
from_left_shift!(c32,  c8,  u8,  2);
from_left_shift!(c16,  c8,  u8,  1);

from_right_shift!(uc8,  uc16, u16, 1);
from_right_shift!(uc8,  uc32, u32, 2);
from_right_shift!(uc8,  uc64, u64, 3);
from_right_shift!(uc16, uc32, u32, 1);
from_right_shift!(uc16, uc64, u64, 2);
from_right_shift!(uc32, uc64, u64, 1);

from_left_shift!(uc64, uc32, u32, 1);
from_left_shift!(uc64, uc16, u16, 2);
from_left_shift!(uc64, uc8,  u8,  3);
from_left_shift!(uc32, uc16, u16, 1);
from_left_shift!(uc32, uc8,  u8,  2);
from_left_shift!(uc16, uc8,  u8,  1);

// Checks

equivalent_solo!(c8);
equivalent_solo!(c16);
equivalent_solo!(c32);
equivalent_solo!(c64);
equivalent_solo!(c128);
equivalent_solo!(uc8);
equivalent_solo!(uc16);
equivalent_solo!(uc32);
equivalent_solo!(uc64);
equivalent_solo!(uc128);

// Algebra

add_same!(c8,  u8,  i8);
add_same!(c16, u16, i16);
add_same!(c32, u32, i32);
add_same!(c64, u64, i64);
add_same!(uc8,  u8,  u8);
add_same!(uc16, u16, u16);
add_same!(uc32, u32, u32);
add_same!(uc64, u64, u64);
sub_same!(c8,  u8,  i8);
sub_same!(c16, u16, i16);
sub_same!(c32, u32, i32);
sub_same!(c64, u64, i64);
sub_same!(uc8,  u8,  u8);
sub_same!(uc16, u16, u16);
sub_same!(uc32, u32, u32);
sub_same!(uc64, u64, u64);