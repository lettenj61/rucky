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

#[macro_export]
macro_rules! import_crates {
    () => { };
    (
        $crate_name:ident $( #[$attr:meta] )*; $( $rest:tt )*
    ) => {
        import_crates!( $crate_name / ( $($attr),* ) );
        import_crates!( $( $rest )* );
    };
    (
        $crate_name:ident; $( $rest:tt ),*
    ) => {
        import_crates!( $crate_name () );
        import_crates!( $( $rest )* );
    };
    // internal: format to unique syntax
    ( $( $crate_name:ident ),* / $attrs:tt ) => {
        $( import_crates!( $crate_name $attrs ); )*
    };
    ( $crate_name:ident ( $($attr:meta),* ) ) => {
        $( #[$attr] )*
        extern crate $crate_name;
    };
}
