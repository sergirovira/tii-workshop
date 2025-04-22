// f1: accepts mutable reference to tuple with two u32s and bool flag. If flag is false, returns mutable
// reference to the first element in the tuple. If flag is true, returns mutable reference to the second
// element in the tuple.
pub fn f1(tuple: &mut (u32, u32, bool)) -> &mut u32 {
    if tuple.2 { &mut tuple.1 } else { &mut tuple.0 }
}

// f2: accepts mutable slice &mut [u32] and number n, returns mutable reference to the n-th element
// in the slice (counting from zero).
pub fn f2(slice: &mut [u32], n: usize) -> &mut u32 {
    &mut slice[n]
}

// f3: accepts slice &mut [u32] and number n, returns mutable reference to the n-th element from the
// end of the slice (counting from zero, i.e. 0 means the last element).

pub fn f3(slice: &mut [u32], n: usize) -> &mut u32 {
    if n >= slice.len() {
        panic!("Index out of bounds");
    }
    &mut slice[slice.len() - 1 - n]
}

// f4: accepts slice &[u32], partitions it into 4 equal (as much as possible) parts, and returns 4
// resulting slices
pub fn f4(slice: &[u32]) -> (&[u32], &[u32], &[u32], &[u32]) {
    let len = slice.len();
    let part_size = len.div_ceil(4); // Round up to ensure all elements are included
    let part_size2 = len / 4;
    let part1 = &slice[0..part_size];
    let part2 = &slice[part_size..part_size + part_size2];
    let part3 = &slice[part_size + part_size2..part_size + 2 * part_size2];
    let part4 = &slice[part_size + 2 * part_size2..];
    (part1, part2, part3, part4)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_f1() {
        let mut tuple = (10, 20, false);
        assert_eq!(*f1(&mut tuple), 10);
        tuple.2 = true;
        assert_eq!(*f1(&mut tuple), 20);
    }
    #[test]
    fn test_f2() {
        let mut slice = [1, 2, 3, 4, 5];
        assert_eq!(*f2(&mut slice, 2), 3);
        *f2(&mut slice, 2) = 10;
        assert_eq!(slice[2], 10);
    }
    #[test]
    fn test_f3() {
        let mut slice = [1, 2, 3, 4, 5];
        assert_eq!(*f3(&mut slice, 0), 5);
        *f3(&mut slice, 0) = 20;
        assert_eq!(slice[4], 20);
    }
    #[test]
    fn test_f4() {
        let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let (part1, part2, part3, part4) = f4(&slice);
        assert_eq!(part1, &[1, 2, 3]);
        assert_eq!(part2, &[4, 5]);
        assert_eq!(part3, &[6, 7]);
        assert_eq!(part4, &[8, 9]);
    }
}
