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
//! for `use` statement, but there are some deviations.
//!
//! The basics like this:
//!
//! ```rust,ignore
//! import!(path::to::module::Item);
//! ```
//!
//! Or import multiple items at once with:
//!
//! ```rust,ignore
//! import! {
//!     nickel { Nickel, HttpRouter };
//!     toml { Value, Table };
//!     std::collections { LinkedList, HashSet };
//! }
//! ```
//!
//! Note that you need to split last segment of module name and block of items with
//! whitespace, instrad of double colons `::` usually used in `use` statement.
//!
//! Glob imports are also supported:
//!
//! ```rust,ignore
//! import!(std::io::prelude *);
//!
//! import! {
//!     rustc_serialize *;
//!     regex *;
//! }
//! ```
//!
//! As you see, separate-with-whitespace rule is also applied to glob-star `*` and
//! path segments as well as multi-item import.
//! That is the limit of this macro.
//!
//! # What you cannot do
//!
//! The macros restricted to:
//!
//! - You can't define alias with `as` keyword when importing an item

mod imports;
mod preluder;
