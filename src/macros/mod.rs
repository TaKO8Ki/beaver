#[macro_export]
macro_rules! define {
    ($struct:ident => ($default:expr) { $($fname:ident -> $ftype:expr),*,}) => {
        pub struct model;
        impl model {
            pub fn f() -> impl Fn(&mut $struct) {
                |m: &mut $struct| {$(m.$fname = $ftype);*}
            }
        }
    };
}

#[macro_export]
macro_rules! hoge {
    ($struct:ident { $($fname:ident: $ftype:expr),*, }) => {};
}

#[macro_export]
macro_rules! def {
    ($($tokens:tt)*) => {
        $crate::parse! {
            tokens = [$($tokens)*],
            imports = [],
            name = unknown,
        }
    };
}

#[macro_export]
macro_rules! parse {
    (
        tokens = [use $($import:tt)::+; $($rest:tt)*],
        imports = [$($imports:tt)*],
        $($args:tt)*
    ) => {
        $crate::parse! {
            tokens = [$($rest)*],
            imports = [$($imports)* use $($import)::+;],
            $($args)*
        }
    };

    (
        tokens = [$name:ident => $($rest:tt)*],
        imports = $imports:tt,
        name = $ignore:tt,
        $($args:tt)*
    ) => {
        $crate::parse! {
            tokens = [$($rest)*],
            imports = $imports,
            name = $name,
            $($args)*
        }
    };

    (
        tokens = [($struct_name:ident { $($fname:ident: $ftype:expr),*, }) {$($rest:tt)*}],
        imports = $imports:tt,
        name = $name:tt,
        $($args:tt)*
    ) => {
        $crate::parse! {
            tokens = [$($rest)*],
            imports = $imports,
            name = $name,
            struct_name = $struct_name,
            default_expr = $struct_name {$($fname: $ftype),*,},
            factory_expr = [],
            factory_ident = [],
        }
    };

    (
        tokens = [$fname:ident -> {$ftype:expr}, $($rest:tt)*],
        imports = $imports:tt,
        name = $name:tt,
        struct_name = $struct_name:tt,
        default_expr = $default:expr,
        factory_expr = [$($factory_expr:tt)*],
        factory_ident = [$($factory_ident:tt)*],
        $($args:tt)*
    ) => {
        $crate::parse! {
            tokens = [$($rest)*],
            imports = $imports,
            name = $name,
            struct_name = $struct_name,
            default_expr = $default,
            factory_expr = [$($factory_expr)* $fname = $ftype;],
            factory_ident = [$($factory_ident)*],
        }
    };

    (
        tokens = [$fname:ident -> <$ftype:ident>, $($rest:tt)*],
        imports = $imports:tt,
        name = $name:tt,
        struct_name = $struct_name:tt,
        default_expr = $default:expr,
        factory_expr = [$($factory_expr:tt)*],
        factory_ident = [$($factory_ident:tt)*],
        $($args:tt)*
    ) => {
        $crate::parse! {
            tokens = [$($rest)*],
            imports = $imports,
            name = $name,
            struct_name = $struct_name,
            default_expr = $default,
            factory_expr = [$($factory_expr)*],
            factory_ident = [$($factory_ident)* $fname = $ftype;],
        }
    };

    (
        tokens = [],
        $($args:tt)*
    ) => {
        $crate::factory_impl! {$($args)*}
    };
}

#[macro_export]
macro_rules! factory_impl {
    (
        imports = [$($imports:tt)*],
        name = $factory_name:ident,
        struct_name = $struct:ident,
        default_expr = $default:expr,
        factory_expr = [$($expr_name:ident = $expr_type:expr;)*],
        factory_ident = [$($ident_name:ident = $ident_type:ident;)*],
    ) => {
        pub mod $factory_name {
            $($imports)*
            use serde::{Deserialize, Serialize};
            use std::marker::PhantomData;
            use std::cell::Cell;
            pub use model as $factory_name;

            pub struct model;

            impl model {
                pub fn new<'a>() -> $crate::factory::Factory<'a, $struct>
                {
                    $crate::factory::Factory {
                        model: serde_json::to_string(&$default).unwrap(),
                        sequence: Cell::new(1),
                        gen_func: Box::new(|m: &mut $struct, n| {$(m.$expr_name = $expr_type(n));*}),
                        _maker: PhantomData,
                    }
                }
            }
        }
    };
}
