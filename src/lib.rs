//#![deny(missing_docs)]
/// The `extra` module organizes the types and additional functions involved with the use of Posits
pub mod extra;
mod certum8;
// Expose c8-c128 to top-level crate
pub use certum8::c8;