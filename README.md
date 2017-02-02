# rucky
I'm feeling rucky!

## about
Small, fuzzy, sketchy, hasty macro library for Rust

Currently provides:
- emulate import statement

## examples

Quick samples below:

```rust
#[macro_use] extern crate rucky;
import_crates! {
    rustc_serialize, toml;
}

import! {
    std::io {stdout, Write};
    std::ffi *;
    toml {Value, Table};
}
```

will be expanded to:

```rust,no_run
extern crate rustc_serialize;
extern crate toml;

use std::io::{stdout, Write};
use std::ffi::*;
```

`import_crates!` macro can only generate `extern crate` statement, note that
it will never import their module members, neither automatically nor explicitly.
So only thing you can put in the macro block is just crate's names.
You also need trailing semicolons `;` to tell the macro where to stop a statement.

Additionaly, you can group crates by attributes with following syntax:

```rust
import_crates! {
    rand, serde_json;
    #[cfg(windows)] winapi, kernel32_sys;
}
```

and the code will be

```rust,ignore
extern crate rand;
extern crate serde_json;
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32_sys;
```

## import modules

For `import!` macro, The syntax used inside is quite similar to the ones
for `use` statement, but there are some deviations.

The basics like this:

```rust,ignore
import!(path::to::module::Item);
```

Or import multiple items at once with:

```rust
import! {
    nickel { Nickel, HttpRouter };
    toml { Value, Table };
    std::collections { LinkedList, HashSet };
}
```

Note that you need to split last segment of module name and block of items with
whitespace, insead of double colons `::` usually used in `use` statement.

Glob imports are also supported:

```rust
import!(std::io::prelude *);

import! {
    rustc_serialize *;
    regex *;
}
```

As you see, separate-with-whitespace rule is also applied to glob-star `*` and
path segments as well as multi-item import.

## limitation

- You can't define alias with `as` keyword when importing an item

## license

The library is released under MIT and Apache 2.0 dual license.
