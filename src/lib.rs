//! Small sketchy macro library.
//!
//! **Many snippets in this document are not tested, please use it with caution.**
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
//! # #[macro_use] extern crate rucky_macros;
//! # fn main() {
//! import_crates! {
//!     rustc_serialize;
//!     toml;
//! }
//! import! {
//!     std::io {stdout, Write};
//!     std::ffi *;
//! #   //toml {Value, Table}; // TODO: failing, should discover what's happening.
//! }
//!
//! import!(std::env::home_dir);
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
//!
//! `import_crates!` macro can only generate `extern crate` statement, note that
//! it will never import their module members, neither automatically nor explicitly.
//! So only thing you can put in the macro block is just crate's names.
//! You also need trailing semicolons `;` to tell the macro where to stop a statement.
//!
//! And for `import!` macro, The syntax used inside is quite similar to the ones
//! you usually write beside `use` keyword, but there are some deviations.
//!
//! The basics are like this:
//!
//! ```text
//! <path to module> { [members] }
//! ```
//!
//! Import several members from a module, you can:
//!
//! ```rust,ignore
//! import! {
//!     nickel { Nickel, HttpRouter };
//!     toml {Value, Table};
//! }
//! ```
//!
//! And glob imports are also supported:
//!

mod imports;
mod preluder;
