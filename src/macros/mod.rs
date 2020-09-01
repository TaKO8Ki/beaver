#[macro_export]
macro_rules! define {
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
        tokens = [$name:ident $($rest:tt)*],
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
        tokens = [($struct_name:ident) $($rest:tt)*],
        imports = $imports:tt,
        name = $name:tt,
        $($args:tt)*
    ) => {
        $crate::parse! {
            tokens = [$($rest)*],
            imports = $imports,
            name = $name,
            struct_name = $struct_name,
            factory_expr = [],
        }
    };

    (
        tokens = [{$($fname:ident -> $ftype:expr),*,}],
        imports = $imports:tt,
        name = $name:tt,
        struct_name = $struct_name:tt,
        factory_expr = [$($factory_expr:tt)*],
        $($args:tt)*
    ) => {
        $crate::parse! {
            tokens = [],
            imports = $imports,
            name = $name,
            struct_name = $struct_name,
            factory_expr = [$($fname = ($ftype);)*],
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
        factory_expr = [$($expr_name:ident = $expr_type:expr;)*],
    ) => {
        #[allow(non_snake_case)]
        pub mod $factory_name {
            $($imports)*
            use serde::{Deserialize, Serialize};
            use std::marker::PhantomData;
            use std::cell::Cell;

            #[allow(non_camel_case_types)]
            pub struct $factory_name;

            impl $factory_name {
                pub(crate) fn new<'a>() -> $crate::factory::Factory<'a, $struct>
                {
                    $crate::factory::Factory {
                        model: serde_json::to_string(&$struct {$($expr_name: $expr_type(1),)*}).unwrap(),
                        sequence: Cell::new(1),
                        gen_func: Box::new(|m: &mut $struct, n| {$(m.$expr_name = $expr_type(n));*}),
                        _maker: PhantomData,
                    }
                }

                pub(crate) fn build<'a>(n: u16) -> $struct
                {
                    $crate::factory::Factory {
                        model: serde_json::to_string(&$struct {$($expr_name: $expr_type(1),)*}).unwrap(),
                        sequence: Cell::new(1),
                        gen_func: Box::new(|m: &mut $struct, n| {$(m.$expr_name = $expr_type(n));*}),
                        _maker: PhantomData,
                    }.build_n(n, |_| {})
                }
            }
        }
    };
}
