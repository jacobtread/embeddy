//! # Embedding
//!
//! To embed a folder simply derive the [`Embedded`] trait on
//! a struct and set the `folder` attribute to the folder path
//! relative to the project Cargo.toml
//!
//! ```
//! use embeddy::Embedded;
//!
//! #[derive(Embedded)]
//! #[folder = "test"]
//! struct MyResources;
//!
//! fn main() {
//!     let file: Option<&'static [u8]> = MyResources::get("test.txt");
//! }
//!
//! ```
pub use embeddy_derive::Embedded;

/// Trait implemented on structures that store
/// embedded resources
pub trait Embedded {
    /// Returns the bytes stored at the provided file path if
    /// there is a file present for that path
    fn get(path: &str) -> Option<&'static [u8]>;
}
