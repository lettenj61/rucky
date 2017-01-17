//! Provides macros to declare or use crates, modules.
//!
//! As their names explain, `import_crates!` and `import!` macros are
//! shortcut to `extern crate` and `use` statement respectively.
//!
//! # Examples
//!
//! Quick samples below:
//!
//! ```
//! #[macro_use] extern crate rucky;
//! # fn main() {
//! import_crates! {
//!     rustc_serialize, toml;
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
//! will be expanded to:
//!
//! ```rust,no_run
//! extern crate rustc_serialize;
//! extern crate toml;
//!
//! use std::io::{stdout, Write};
//! use std::ffi::*;
//! ```
//!
//! # Import crates
//!
//! `import_crates!` macro can only generate `extern crate` statement, note that
//! it will never import their module members, neither automatically nor explicitly.
//! So only thing you can put in the macro block is just crate's names.
//! You also need trailing semicolons `;` to tell the macro where to stop a statement.
//!
//! Additionaly, you can group crates by attributes with following syntax:
//!
//! ```rust,ignore
//! import_crates! {
//!     rand, serde_json;
//!     #[cfg(windows)] winapi, kernel32_sys;
//! }
//! ```
//!
//! and the code will be
//!
//! ```rust,ignore
//! extern crate rand;
//! extern crate serde_json;
//! #[cfg(windows)] extern crate winapi;
//! #[cfg(windows)] extern crate kernel32_sys;
//! ```
//!
//! # Import modules
//!
//! For `import!` macro, The syntax used inside is quite similar to the ones
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
//! whitespace, insead of double colons `::` usually used in `use` statement.
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
//! - You can't define alias with `as` keyword when importing an item

/// Macro to generate `use` statement at once.
#[macro_export]
macro_rules! import {
    () => { };
    ( $pkg:path ) => {
        use $pkg;
    };
    ( $root:ident * ) => {
        use $root :: *;
    };
    ( $root:ident :: $($tail:ident)::+ * ) => {
        use $root :: $( $tail )::* :: *;
    };
    ( $pkg:path ; $( $rest:tt )* ) => {
        use $pkg;
        import!( $( $rest )* );
    };
    (
        $root:ident { $( $member:ident ),+ } ;
        $( $rest:tt )*
    ) => {
        use $root :: {
            $( $member ),*
        } ;
        import!( $( $rest )* );
    };
    (
        $root:ident :: $($tail:ident)::+ *;
        $( $rest:tt )*
    ) => {
        use $root :: $( $tail )::* :: *;
        import!( $( $rest )* );
    };
    (
        $root:ident :: $($tail:ident)::+
        { $( $member:ident ),+ } ;
        $( $rest:tt )*
    ) => {
        use $root :: $( $tail )::* :: {
            $( $member ),*
        } ;
        import!( $( $rest )* );
    };
}

/// Macro to generate `extern crate` statement at once.
#[macro_export]
macro_rules! import_crates {
    ( ) => { };
    // single crate with no attribute
    ( $krate:ident ; $( $rest:tt )* ) => {
        import_crates!( $krate () );
        import_crates!( $( $rest )* );
    };
    // crates with no preceeding attribute
    ( $($krate:ident),+ ; $( $rest:tt )* ) => {
        import_crates!( $( $krate ),* / () );
        import_crates!( $( $rest )* );
    };
    // grouped by attributes
    ( $(#[$attr:meta])+ $($krate:ident),+ ; $( $rest:tt )* ) => {
        import_crates!( $( $krate ),* / ( $($attr),* ) );
        import_crates!( $( $rest )* );
    };
    // tupled intermediate expression
    ( $( $krate:ident ),* / $attrs:tt ) => {
        $( import_crates!( $krate $attrs ); )*
    };
    // resolved statement
    ( $krate:ident ( $($attr:meta),* ) ) => {
        $( #[$attr] )*
        extern crate $krate;
    };
}
