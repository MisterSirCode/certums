#[macro_export]
/// Negate a certum
macro_rules! negate {
    ($target:ident) => {
        impl Neg for $target {
            type Output = $target;
            fn neg(self) -> Self {
                Self { bits: !self.bits + 1 }
            }
        }
    }
}

#[macro_export]
/// Convert between two certum variants by directly replacing bits
macro_rules! from_direct {
    ($source:ident, $target:ident) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                $target { bits: value.bits }
            }
        }
    }
}

#[macro_export]
/// Convert between two certum variants through a left shift then a cast
macro_rules! from_left_shift {
    ($source:ident, $target:ident, $cast:ty, $bits:expr, $shift:expr) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                $target { bits: (value.bits as $cast) << ($bits - $shift) }
            }
        }
    }
}

#[macro_export]
/// Convert between two certum variants through a cast then a right shift
macro_rules! from_right_shift {
    ($source:ident, $target:ident, $cast:ty, $bits:expr, $shift:expr) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                $target { bits: (value.bits >> ($bits - $shift)) as $cast }
            }
        }
    }
}

#[macro_export]
/// Equivalency Checks
macro_rules! equivalent_solo {
    ($target:ident) => {
        impl PartialEq for $target {
            fn eq(&self, other: &Self) -> bool {
                if self.bits == other.bits { true }
                else { false } 
            }
        }

        impl Eq for $target { }
    }
}

#[macro_export]
/// Equivalency Checks
macro_rules! equivalent_other {
    ($target:ident, $other:ident) => {
        impl PartialEq<$other> for $target {
            fn eq(&self, other: &Self) -> bool {
                if self.bits == other.bits { true }
                else { false } 
            }
        }
    }
}

#[macro_export]
/// Addition for types
macro_rules! add_same {
    ($target:ident, $uint:ty, $sint:ty) => {
        impl Add for $target {
            type Output = $target;
            fn add(self, rhs: Self) -> Self {
                let bits = <$sint>::saturating_add(self.bits as $sint, rhs.bits as $sint);
                $target { bits: bits as $uint }
            }
        }
    }
}

#[macro_export]
/// Addition for types
macro_rules! sub_same {
    ($target:ident, $uint:ty, $sint:ty) => {
        impl Sub for $target {
            type Output = $target;
            fn sub(self, rhs: Self) -> Self {
                let bits = <$sint>::saturating_sub(self.bits as $sint, rhs.bits as $sint);
                $target { bits: bits as $uint }
            }
        }
    }
}

#[macro_export]
/// General float casts
macro_rules! float_casts {
    ($target:ident, $uint:ty) => {
        impl From<&$target> for f64 {
            /// Convert to a 64-bit Float
            fn from(value: &$target) -> Self {
                f64::from(*value)
            }
        }

        impl From<$target> for f32 {
            /// Convert to a 32-bit Float
            fn from(value: $target) -> Self {
                f64::from(value) as f32
            }
        }

        impl From<&$target> for f32 {
            /// Convert to a 32-bit Float
            fn from(value: &$target) -> Self {
                f64::from(*value) as f32
            }
        }

        impl From<$uint> for $target {
            /// Convert from a UInt
            fn from(bits: $uint) -> Self {
                $target { bits }
            }
        }

        impl From<&$uint> for $target {
            /// Convert from a UInt
            fn from(value: &$uint) -> Self {
                $target::from(*value)
            }
        }

        impl From<f32> for $target {
            /// Convert from a 32-Bit Float
            fn from(val: f32) -> Self {
                $target::from(val as f64)
            }
        }
    }
}

#[macro_export]
/// Float conversion for signed certums
macro_rules! float_convert_sc {
    ($target:ident, $uint:ty, $sint:ty, $scale:expr, $dec:expr, $lim:expr) => {
        impl From<$target> for f64 {
            /// Convert to a 64-bit Float
            fn from(value: $target) -> Self {
                (value.bits as $sint) as f64 / (<$uint>::from(1u8) << ($scale - $dec)) as f64
            }
        }

        impl From<f64> for $target {
            /// Convert from a 64-bit Float
            fn from(val: f64) -> Self {
                let (sgn, int, frc) = f64_split(val.clamp($target::MINF, $target::MAXF));
                let sign = (sgn as $uint) << ($scale - 1);
                // Combine integer and fraction parts
                let combined = ((int as $uint) << ($scale - $dec)) | $target::u64_round(frc >> $dec);
                // Clamp off for sign and add sign bit
                // 2^(bits - 1) - 1
                let mut checked = (combined & $lim);
                if sign > 0 { checked = !checked + 1 }
                $target { bits: sign | checked }
            }
        }
    };
}

#[macro_export]
/// Float conversion for unsigned certums
macro_rules! float_convert_uc {
    ($target:ident, $uint:ty, $scale:expr, $dec:expr, $lim:expr) => {
        impl From<$target> for f64 {
            /// Convert to a 64-bit Float
            fn from(value: $target) -> Self {
                let (int, frc) = value.components();
                let float_frc = (frc as f64) / f64::powi(2f64, $scale); // MSB-Shifted fraction / 2^Bits
                (int as f64) + float_frc // Add integer and fraction
            }
        }

        impl From<f64> for $target {
            /// Convert from a 64-bit Float
            fn from(val: f64) -> Self {
                let (_sgn, int, frc) = f64_split(val.clamp($target::MINF, $target::MAXF));
                // Combine integer and fraction parts
                let bits = ((int as $uint) << ($scale - $dec)) | $target::u64_round(frc >> $dec);
                $target { bits }
            }
        }
    };
}

#[macro_export]
// Float conversion for types > 64 bit
macro_rules! lossy_float_convert {
    ($target:ident, $fromtype:ident) => {
        impl From<$target> for f64 {
            /// Convert to a 64-bit Float
            fn from(value: $target) -> Self {
                f64::from($fromtype::from(value))
            }
        }

        impl From<f64> for $target {
            /// Convert from a 64-bit Float
            fn from(val: f64) -> Self {
                $target::from($fromtype::from(val))
            }
        }
    };
}


// impl Add for $target {
//     type Output = $target;
//     fn add(self, rhs: Self) -> Self::Output {
//         let (sgn, int, frc) = self.components();
//         let (psgn, pint, pfrc) = rhs.components();
//         let cbfrc = (frc as i16) * (sgn as i16) + (pfrc as i16) * (psgn as i16);
//         let fcarry = (cbfrc >> 8) as i8;
//         let cbint = (int as i8) * sgn + (pint as i8) * psgn + fcarry;
//         let sgn_bit = ((cbint >> 7) as $uint) << 7;
//         let int_bit = ((cbint & 0x7F) as $uint) << 6;
//         let frc_bit = (cbfrc as $uint) >> 2;
//         $target { bits: sgn_bit | int_bit | frc_bit }
//     }
// }
