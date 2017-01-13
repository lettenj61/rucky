//! Small sketchy macro library.
//!
//! This crate currently provides these macros:
//!
//! # Import crates and modules
//!
//! As their names explain, `import_crates!` and `import!` macros are
//! shortcut to `extern crate` and `use` statement respectively.
//!
//! Quick samples below:
//!
//! ```
//! # #![allow(unused_import)]
//! # #[macro_use] extern crate rucky_macros;
//! # fn main() {
//! import_crates! {
//!     rustc_serialize;
//!     toml;
//! }
//! import! {
//!     std::io {stdout, Write};
//!     std::ffi *;
//! }
//! # }
//! ```
//!
//! Will be expanded into:
//!
//! ```rust,no_run
//! extern crate rustc_serialize;
//! extern crate toml;
//!
//! use std::io::{stdout, Write};
//! use std::ffi::*;
//! ```

mod imports;
mod preluder;
