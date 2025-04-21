struct BigUint4096 {
    data: [u64; 64],
}

impl BigUint4096 {
    fn new() -> Self {
        BigUint4096 { data: [0; 64] }
    }
}
impl std::ops::Add for BigUint4096 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = BigUint4096::new();
        let mut carry = 0;
        for i in 0..64 {
            let (sum, overflow) = self.data[i].overflowing_add(other.data[i]);
            result.data[i] = sum.wrapping_add(carry);
            carry = if overflow { 1 } else { 0 };
        }
        result
    }
}
impl std::ops::Sub for BigUint4096 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = BigUint4096::new();
        let mut borrow = 0;
        for i in 0..64 {
            let (diff, overflow) = self.data[i].overflowing_sub(other.data[i]);
            result.data[i] = diff.wrapping_sub(borrow);
            borrow = if overflow { 1 } else { 0 };
        }
        result
    }
}

impl std::ops::Mul for BigUint4096 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = BigUint4096::new();
        for i in 0..64 {
            for j in 0..64 {
                if i + j < 64 {
                    let (prod, overflow) = self.data[i].overflowing_mul(other.data[j]);
                    result.data[i + j] = result.data[i + j].wrapping_add(prod);
                }
            }
        }
        result
    }
}

//implement convertion methods from hex encoded &str and to hex encoded String
impl From<&str> for BigUint4096 {
    fn from(hex_str: &str) -> Self {
        let mut result = BigUint4096::new();
        let bytes = hex::decode(hex_str).expect("Invalid hex string");
        for (i, byte) in bytes.iter().enumerate() {
            if i < 64 {
                result.data[i] = *byte as u64;
            }
        }
        result
    }
}

impl From<BigUint4096> for String {
    fn from(biguint: BigUint4096) -> Self {
        let mut hex_str = String::new();
        for &byte in biguint.data.iter() {
            hex_str.push_str(&format!("{:02x}", byte));
        }
        hex_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = BigUint4096::new();
        let b = BigUint4096::new();
        let c = a + b;
        assert_eq!(c.data, [0; 64]);
    }

    #[test]
    fn test_sub() {
        let a = BigUint4096::new();
        let b = BigUint4096::new();
        let c = a - b;
        assert_eq!(c.data, [0; 64]);
    }

    #[test]
    fn test_mul() {
        let a = BigUint4096::new();
        let b = BigUint4096::new();
        let c = a * b;
        assert_eq!(c.data, [0; 64]);
    }

    // #[test]
    // fn test_from_hex() {
    //     let hex_str = "a3f9b7d204e68c1f5b0e9a7c";
    //     let biguint = BigUint4096::from(hex_str);
    //     assert_eq!(
    //         biguint.data[0],
    //         "a3f9b7d204e68c1f5b0e9a7c".parse::<u64>().unwrap()
    //     );
    // }
}
