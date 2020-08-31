use crate::factory::new;

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
        tokens = [($default:expr) { $($rest:tt)* }],
        imports = $imports:tt,
        name = $name:tt,
        $($args:tt)*
    ) => {
        $crate::parse! {
            tokens = [],
            imports = $imports,
            name = $name,
            default = $default,
            factory = $($rest)*
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
        default = $default:expr,
        factory = $($fname:ident -> $ftype:expr),*,
    ) => {
        pub mod $factory_name {
            $($imports)*

            pub struct model;

            impl model {
                fn build() {
                    $crate::factory::new($default, |m, n| {$(m.$fname = $ftype);*});
                }
            }
        }
    };
}
