//#![deny(missing_docs)]
/// The `extra` module organizes the types and additional functions involved with the use of Posits
pub mod utils;
pub mod certum;
pub mod macros;

#[cfg(test)]
pub mod tests;
// pub mod dimid;
// pub mod quarta;
// pub mod acute;
// Expose c8-c128 to top-level crate
pub use certum::certum8::c8;
pub use certum::certum16::c16;
pub use certum::certum32::c32;
pub use certum::certum64::c64;

pub use certum::u_certum8::uc8;
pub use certum::u_certum16::uc16;
pub use certum::u_certum32::uc32;
pub use certum::u_certum64::uc64;