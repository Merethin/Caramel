pub mod format;

#[cfg(feature = "ns-xml")]
pub mod xml;

#[cfg(feature = "ns-api")]
pub mod api;

mod ua;
pub use ua::*;