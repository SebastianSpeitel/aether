#[cfg(feature = "embedded")]
pub mod pin;

#[cfg(feature = "embedded")]
pub use pin::PinDriver;
