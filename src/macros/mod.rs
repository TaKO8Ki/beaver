#[macro_export]
macro_rules! define {
    ($name:ident ($struct:path) { $($fname:ident -> $ftype:expr),*,}) => {
        // impl $name {
        //     fn field_names() -> &'static [&'static str] {
        //         static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
        //         NAMES
        //     }
        // }

        pub fn clo() -> impl Fn(&mut $struct) {
            |$name: &mut $struct| {$(($name).$fname = $ftype);*}
        }
    };
}

#[macro_export]
macro_rules! def {
    ($($tokens:tt)*) => {
        tokens = [$($tokens)*],
        imports = [],
        meta = [],
        sql_name = unknown,
        name = unknown,
        schema = public,
        primary_key = id,
    };
}
