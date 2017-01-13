#![allow(unused_imports)]

#[macro_use]
extern crate rucky_macros;

import_crates! {
    toml;
    rustc_serialize;
    rand #[cfg(feature = "test_attr")];
}

#[test]
fn rustc_compiles_this() {
    // This function will be successfully compiled
    // if macro expanded correctly.
}

#[test]
fn specify_member() {
    import! {
        rustc_serialize::json {Json, Builder, AsJson};
        std::path {Path, PathBuf};
    }
}

#[cfg(feature = "test_attr")]
#[test]
fn attributed_import() {
    let t = rand::random::<(f64, char)>();
    println!("{:?}", t);
}
