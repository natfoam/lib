#[inline]
pub const fn is_set(v: usize, i: u8) -> bool{
    ((v >> i) & 1) != 0
}

#[inline]
pub const fn unset(v: usize, i: u8) -> usize {
    v & !(1 << i)
}

#[inline]
pub const fn log2(v: usize) -> u32 {
    usize::BITS - 1 - v.leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::is_set;

    #[test]
    fn test_is_set() {
        assert!(!is_set(0, 0));
        assert!(!is_set(0b10, 0));
        assert!(is_set(1, 0));
        assert!(is_set(0b1110_1111, 0));
        //
        assert!(!is_set(0, 1));
        assert!(!is_set(1, 1));
        assert!(is_set(0b10, 1));
        assert!(is_set(0b1010_1010, 1));
        assert!(is_set(0b1010_1011, 1));
        //
        assert!(!is_set(0, (usize::BITS - 1) as u8));
        assert!(is_set(usize::MAX, (usize::BITS - 1) as u8));
        assert!(is_set(usize::MAX - (usize::MAX >> 1), (usize::BITS - 1) as u8));
    }
}