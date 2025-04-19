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
}
