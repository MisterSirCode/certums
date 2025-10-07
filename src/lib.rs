//#![deny(missing_docs)]
/// The `extra` module organizes the types and additional functions involved with the use of Posits
pub mod utils;
pub mod extra;
pub mod certum;
pub mod dimid;
pub mod quarta;
pub mod acute;
// Expose c8-c128 to top-level crate
pub use certum::certum8::c8;