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
    ($source:ident, $target:ident, $cast:ty, $shift:expr) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                $target { bits: (value.bits << $shift) as $cast }
            }
        }
    }
}

#[macro_export]
/// Convert between two certum variants through a cast then a right shift
macro_rules! from_right_shift {
    ($source:ident, $target:ident, $cast:ty, $shift:expr) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                $target { bits: (value.bits as $cast) >> $shift }
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
macro_rules! add_same_signed {
    ($target:ident, $uint:ty, $sint:ty, $duint:ty, $dsint:ty, $scale:expr, $dec:expr, $lim:expr) => {
        impl Add for $target {
            type Output = $target;
            fn add(self, rhs: Self) -> Self::Output {
                if (self == $target::MAX) | (rhs == $target::MAX) { return $target::MAX };
                let bits = <$sint>::saturating_add(self.bits as $sint, rhs.bits as $sint);
                $target { bits: bits as $uint }
            }
        }
    }
}


// impl Add for c8 {
//     type Output = c8;
//     fn add(self, rhs: Self) -> Self::Output {
//         let (sgn, int, frc) = self.components();
//         let (psgn, pint, pfrc) = rhs.components();
//         let cbfrc = (frc as i16) * (sgn as i16) + (pfrc as i16) * (psgn as i16);
//         let fcarry = (cbfrc >> 8) as i8;
//         let cbint = (int as i8) * sgn + (pint as i8) * psgn + fcarry;
//         let sgn_bit = ((cbint >> 7) as u8) << 7;
//         let int_bit = ((cbint & 0x7F) as u8) << 6;
//         let frc_bit = (cbfrc as u8) >> 2;
//         c8 { bits: sgn_bit | int_bit | frc_bit }
//     }
// }
