#[macro_export]
macro_rules! define {
    ($($tokens:tt)*) => {
        $crate::beaver_parse! {
            tokens = [$($tokens)*],
            imports = [],
            name = unknown,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! beaver_parse {
    (
        tokens = [use $($import:tt)::+; $($rest:tt)*],
        imports = [$($imports:tt)*],
        $($args:tt)*
    ) => {
        $crate::beaver_parse! {
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
        $crate::beaver_parse! {
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
        $crate::beaver_parse! {
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
        $crate::beaver_parse! {
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
#[doc(hidden)]
macro_rules! factory_impl {
    (
        imports = [$($imports:tt)*],
        name = $factory_name:ident,
        struct_name = $struct:ident,
        factory_expr = [$($expr_name:ident = $expr_type:expr;)*],
    ) => {
        $($imports)*

        pub struct $factory_name;

        impl $factory_name {
            pub(crate) fn new<'a>() -> $crate::factory::Factory<'a, $struct>
            {
                $crate::factory::new(
                    $struct {$($expr_name: $expr_type(1),)*},
                    Box::new(|m: &mut $struct, n| {$(m.$expr_name = $expr_type(n));*})
                )
            }

            pub(crate) fn build<'a>(n: u16) -> $struct
            {
                $crate::factory::new(
                    $struct {$($expr_name: $expr_type(1),)*},
                    Box::new(|m: &mut $struct, n| {$(m.$expr_name = $expr_type(n));*})).build_n(n, |_| {}
                )
            }

            pub(crate) fn build_list<'a>(number: u16, n: u16) -> Vec<$struct>
            {
                $crate::factory::new(
                    $struct {$($expr_name: $expr_type(1),)*},
                    Box::new(|m: &mut $struct, n| {$(m.$expr_name = $expr_type(n));*})).build_list_n(number, n, |_| {}
                )
            }
        }
    };
}
