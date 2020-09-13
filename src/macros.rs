/// Defines a [Factory](struct.Factory.html).
///
/// Example usage
/// -------------
/// ```rust
/// use chrono::{NaiveDate, NaiveDateTime};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// pub struct Post {
///     id: u16,
///     title: String,
///     approved: bool,
///     created_at: NaiveDateTime,
/// }
///
/// beaver::define! {
///     PostFactory (Post) {
///         id -> |n| n,
///         title -> |n| format!("{}", n),
///         approved -> |_| false,
///         created_at -> |_| NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0),
///     }
/// }
/// ```
///
/// If you want to use sub factory, you can use `build(n)` like the following:
///
/// ```rust
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// pub struct File {
///     id: u16,
///     path: String,
/// }
///
/// #[derive(Serialize, Deserialize)]
/// pub struct User {
///     id: u16,
///     name: String,
///     file: File,
/// }
///
/// beaver::define! {
///     UserFactory (User) {
///         id -> |n| n,
///         name -> |n| format!("user-{}", n),
///         file -> |n| FileFactory::build(n),
///     }
/// }
///
/// beaver::define! {
///     FileFactory (File) {
///         id -> |n| n,
///         path -> |n| format!("path/to/file-{}", n),
///     }
/// }
/// ```
///
/// If you want to use a vector of sub factories, you can use `build_list(number, n)` like the following:
///
/// ```rust
/// use chrono::{NaiveDate, NaiveDateTime};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// pub struct Post {
///     id: u16,
///     title: String,
///     approved: bool,
///     tags: Vec<Tag>,
/// }
///
/// #[derive(Serialize, Deserialize)]
/// pub struct Tag {
///     id: u16,
///     name: String,
/// }
///
/// beaver::define! {
///     PostFactory (Post) {
///         id -> |n| n,
///         title -> |n| format!("post-{}", n),
///         approved -> |_| true,
///         tags -> |n| TagFactory::build_list(3, n),
///     }
/// }
///
/// beaver::define! {
///     TagFactory (Tag) {
///         id -> |n| beaver::sequence(100, n),
///         name -> |n| format!("tag-{}", n),
///     }
/// }
/// ```
#[macro_export]
macro_rules! define {
    ($($tokens:tt)*) => {
        $crate::beaver_parse! {
            tokens = [$($tokens)*],
            factory_name = unknown,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! beaver_parse {
    (
        tokens = [pub $factory_name:ident $($rest:tt)*],
        factory_name = $ignore:tt,
        $($args:tt)*
    ) => {
        $crate::beaver_parse! {
            tokens = [$($rest)*],
            factory_name = $factory_name,
            public = true,
            $($args)*
        }
    };

    (
        tokens = [$factory_name:ident $($rest:tt)*],
        factory_name = $ignore:tt,
        $($args:tt)*
    ) => {
        $crate::beaver_parse! {
            tokens = [$($rest)*],
            factory_name = $factory_name,
            public = false,
            $($args)*
        }
    };

    (
        tokens = [($struct_name:ident) $($rest:tt)*],
        factory_name = $factory_name:tt,
        public = $public:ident,
        $($args:tt)*
    ) => {
        $crate::beaver_parse! {
            tokens = [$($rest)*],
            factory_name = $factory_name,
            public = $public,
            struct_name = $struct_name,
            fields = [],
        }
    };

    (
        tokens = [{$($fname:ident -> $fvalue:expr),*,}],
        factory_name = $factory_name:tt,
        public = $public:ident,
        struct_name = $struct_name:tt,
        fields = [$($ignore:tt)*],
        $($args:tt)*
    ) => {
        $crate::beaver_parse! {
            tokens = [],
            factory_name = $factory_name,
            public = $public,
            struct_name = $struct_name,
            fields = [$($fname = ($fvalue);)*],
        }
    };

    (
        tokens = [],
        $($args:tt)*
    ) => {
        $crate::beaver_factory_impl! {$($args)*}
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! beaver_factory_impl {
    (
        factory_name = $factory_name:ident,
        public = false,
        struct_name = $struct:ident,
        fields = [$($fname:ident = $fvalue:expr;)*],
    ) => {
        pub struct $factory_name;

        impl $factory_name {
            fn new<'a>() -> $crate::Factory<'a, $struct>
            {
                $crate::new(
                    $struct {$($fname: $fvalue(1),)*},
                    Box::new(|m: &mut $struct, n| {$(m.$fname = $fvalue(n));*})
                )
            }

            fn build<'a>(n: u16) -> $struct
            {
                $crate::new(
                    $struct {$($fname: $fvalue(1),)*},
                    Box::new(|m: &mut $struct, n| {$(m.$fname = $fvalue(n));*})
                ).build_n(n, |_| {})
            }

            fn build_list<'a>(number: u16, n: u16) -> Vec<$struct>
            {
                $crate::new(
                    $struct {$($fname: $fvalue(1),)*},
                    Box::new(|m: &mut $struct, n| {$(m.$fname = $fvalue(n));*})
                ).build_list_n(number, n, |_| {})
            }
        }
    };

    (
        factory_name = $factory_name:ident,
        public = true,
        struct_name = $struct:ident,
        fields = [$($fname:ident = $fvalue:expr;)*],
    ) => {
        pub struct $factory_name;

        impl $factory_name {
            pub fn new<'a>() -> $crate::Factory<'a, $struct>
            {
                $crate::new(
                    $struct {$($fname: $fvalue(1),)*},
                    Box::new(|m: &mut $struct, n| {$(m.$fname = $fvalue(n));*})
                )
            }

            pub fn build<'a>(n: u16) -> $struct
            {
                $crate::new(
                    $struct {$($fname: $fvalue(1),)*},
                    Box::new(|m: &mut $struct, n| {$(m.$fname = $fvalue(n));*})
                ).build_n(n, |_| {})
            }

            pub fn build_list<'a>(number: u16, n: u16) -> Vec<$struct>
            {
                $crate::new(
                    $struct {$($fname: $fvalue(1),)*},
                    Box::new(|m: &mut $struct, n| {$(m.$fname = $fvalue(n));*})
                ).build_list_n(number, n, |_| {})
            }
        }
    };
}
