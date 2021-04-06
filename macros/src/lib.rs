#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $value:expr, )* ) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($key, $value);
            )*
            hm
        }
    };
    ( $( $key:expr => $value:expr ),* ) => {
        ::macros::hashmap!($( $key => $value, )*)
    };
}
