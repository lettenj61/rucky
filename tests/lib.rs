#![allow(unused_imports)]

#[macro_use]
extern crate rucky;

import_crates! {
    toml;
    regex;
    rustc_serialize;
}

#[test]
fn rustc_compiles_this() {
    // This function will be successfully compiled
    // if macro expanded correctly.
}

#[test]
fn specify_member() {
    import! {
        std::io *;
        toml {Value, Table};
        rustc_serialize::json {Json, Builder, AsJson};
        std::path {Path, PathBuf};
    }
}

#[test]
fn use_imported_member() {
    import!(std::collections::BTreeMap);

    let mut map = BTreeMap::<i32, Vec<f64>>::new();
    map.insert(5, vec![2.0, 4.5]);
    assert_eq!(map.get(&5).unwrap().as_slice(), &[2.0, 4.5]);
}

#[test]
fn one_liner_glob_import() {
    import!(std::io::prelude *);
    import!(regex *);
}
