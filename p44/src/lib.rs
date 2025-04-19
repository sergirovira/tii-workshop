use std::collections::HashMap;

#[macro_export]
macro_rules! hash_map {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

macro_rules! new_biguints {
    ($($name:ident, $size:expr);* $(;)?) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name {
                pub data: [u64; $size],
            }

            impl $name {
                pub fn new() -> Self {
                    $name { data: [0; $size] }
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_map() {
        let map: HashMap<i32, bool> = hash_map!(
            42 => true,
            64 => false,
            128 => true,
            256 => false,
            512 => true,
            1024 => false,
        );

        assert_eq!(map.get(&42), Some(&true));
        assert_eq!(map.get(&64), Some(&false));
        assert_eq!(map.get(&128), Some(&true));
        assert_eq!(map.get(&256), Some(&false));
        assert_eq!(map.get(&512), Some(&true));
        assert_eq!(map.get(&1024), Some(&false));
    }

    #[test]
    fn test_new_biguints() {
        new_biguints!(BigUint64, 64; BigUint128, 128; BigUint256, 256; BigUint512, 512; BigUint1024, 1024; BigUint2048, 2048; BigUint4096, 4096);

        let big_uint_64 = BigUint64::new();
        let big_uint_128 = BigUint128::new();
        let big_uint_256 = BigUint256::new();
        let big_uint_512 = BigUint512::new();
        let big_uint_1024 = BigUint1024::new();
        let big_uint_2048 = BigUint2048::new();
        let big_uint_4096 = BigUint4096::new();

        assert_eq!(big_uint_64.data.len(), 64);
        assert_eq!(big_uint_128.data.len(), 128);
        assert_eq!(big_uint_256.data.len(), 256);
        assert_eq!(big_uint_512.data.len(), 512);
        assert_eq!(big_uint_1024.data.len(), 1024);
        assert_eq!(big_uint_2048.data.len(), 2048);
        assert_eq!(big_uint_4096.data.len(), 4096);
    }
}
