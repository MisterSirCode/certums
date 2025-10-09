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
