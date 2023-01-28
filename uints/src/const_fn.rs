#[inline]
pub const fn is_set(v: usize, i: u8) -> bool {
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
        assert!(is_set(
            usize::MAX - (usize::MAX >> 1),
            (usize::BITS - 1) as u8
        ));
    }

    const fn trailing_zeros_8e((mut v, mut extra): (u8, u8)) -> u8 {
        {
            let lo = v & 0xF;
            v = if lo == 0 {
                extra += 4;
                v >> 4
            } else {
                lo
            }
        };
        {
            let lo = v & 0x3;
            v = if lo == 0 {
                extra += 2;
                v >> 2
            } else {
                lo
            }
        };
        extra + (if v & 1 == 0 { 2 - (v >> 1) } else { 0 })
    }

    const fn trailing_zeros_8(v: u8) -> u8 {
        trailing_zeros_8e((v, 0))
    }

    const fn trailing_zeros_16e((v, extra): (u16, u8)) -> u8 {
        trailing_zeros_8e({
            let lo = v as u8;
            if lo == 0 {
                ((v >> 8) as u8, extra + 8)
            } else {
                (lo, extra)
            }
        })
    }

    const fn trailing_zeros_16(v: u16) -> u8 {
        trailing_zeros_16e((v, 0))
    }

    const fn trailing_zeros_32e((v, extra): (u32, u8)) -> u8 {
        trailing_zeros_16e({
            let lo = v as u16;
            if lo == 0 {
                ((v >> 16) as u16, extra + 16)
            } else {
                (lo, extra)
            }
        })
    }

    const fn trailing_zeros_32(v: u32) -> u8 {
        trailing_zeros_32e((v, 0))
    }

    const fn trailing_zeros_64(v: u64) -> u8 {
        trailing_zeros_32e({
            let lo = v as u32;
            if lo == 0 {
                ((v >> 32) as u32, 32)
            } else {
                (lo, 0)
            }
        })
    }

    #[test]
    fn soft_test64() {
        assert_eq!(trailing_zeros_64(0), 64);
        assert_eq!(trailing_zeros_64(1), 0);
        assert_eq!(trailing_zeros_64(2), 1);
        assert_eq!(trailing_zeros_64(3), 0);
        assert_eq!(trailing_zeros_64(4), 2);
        assert_eq!(trailing_zeros_64(7), 0);
        assert_eq!(trailing_zeros_64(8), 3);
        assert_eq!(trailing_zeros_64(0x10), 4);
        assert_eq!(trailing_zeros_64(0x20), 5);
        assert_eq!(trailing_zeros_64(0x40), 6);
        assert_eq!(trailing_zeros_64(0x80), 7);
        assert_eq!(trailing_zeros_64(0x8000_0000_0000_0000), 63);
        assert_eq!(trailing_zeros_64(0x8000_0000_0000_0001), 0);
        assert_eq!(trailing_zeros_64(0xC000_0000_0000_0000), 62);
        assert_eq!(trailing_zeros_64(0x4000_0000_0000_0000), 62);
        assert_eq!(trailing_zeros_64(0xE000_0000_0000_0000), 61);
        assert_eq!(trailing_zeros_64(0xF000_0000_0000_0000), 60);
    }

    #[test]
    fn soft_test32() {
        assert_eq!(trailing_zeros_32(0), 32);
        assert_eq!(trailing_zeros_32(1), 0);
        assert_eq!(trailing_zeros_32(2), 1);
        assert_eq!(trailing_zeros_32(3), 0);
        assert_eq!(trailing_zeros_32(4), 2);
        assert_eq!(trailing_zeros_32(7), 0);
        assert_eq!(trailing_zeros_32(8), 3);
        assert_eq!(trailing_zeros_32(0x10), 4);
        assert_eq!(trailing_zeros_32(0x20), 5);
        assert_eq!(trailing_zeros_32(0x40), 6);
        assert_eq!(trailing_zeros_32(0x80), 7);
        assert_eq!(trailing_zeros_32(0x8000_0000), 31);
        assert_eq!(trailing_zeros_32(0x8000_0001), 0);
        assert_eq!(trailing_zeros_32(0xC000_0000), 30);
        assert_eq!(trailing_zeros_32(0x4000_0000), 30);
        assert_eq!(trailing_zeros_32(0xE000_0000), 29);
        assert_eq!(trailing_zeros_32(0xF000_0000), 28);
    }

    #[test]
    fn soft_test16() {
        assert_eq!(trailing_zeros_16(0), 16);
        assert_eq!(trailing_zeros_16(1), 0);
        assert_eq!(trailing_zeros_16(2), 1);
        assert_eq!(trailing_zeros_16(3), 0);
        assert_eq!(trailing_zeros_16(4), 2);
        assert_eq!(trailing_zeros_16(7), 0);
        assert_eq!(trailing_zeros_16(8), 3);
        assert_eq!(trailing_zeros_16(0x10), 4);
        assert_eq!(trailing_zeros_16(0x20), 5);
        assert_eq!(trailing_zeros_16(0x40), 6);
        assert_eq!(trailing_zeros_16(0x80), 7);
        assert_eq!(trailing_zeros_16(0x8000), 15);
        assert_eq!(trailing_zeros_16(0x8001), 0);
        assert_eq!(trailing_zeros_16(0xC000), 14);
        assert_eq!(trailing_zeros_16(0x4000), 14);
        assert_eq!(trailing_zeros_16(0xE000), 13);
        assert_eq!(trailing_zeros_16(0xF000), 12);
    }

    #[test]
    fn soft_test8() {
        assert_eq!(trailing_zeros_8(0), 8);
        assert_eq!(trailing_zeros_8(1), 0);
        assert_eq!(trailing_zeros_8(2), 1);
        assert_eq!(trailing_zeros_8(3), 0);
        assert_eq!(trailing_zeros_8(4), 2);
        assert_eq!(trailing_zeros_8(7), 0);
        assert_eq!(trailing_zeros_8(8), 3);
        assert_eq!(trailing_zeros_8(0x10), 4);
        assert_eq!(trailing_zeros_8(0x20), 5);
        assert_eq!(trailing_zeros_8(0x40), 6);
        assert_eq!(trailing_zeros_8(0x80), 7);
        assert_eq!(trailing_zeros_8(0x82), 1);
        assert_eq!(trailing_zeros_16(0x81), 0);
        assert_eq!(trailing_zeros_16(0xC0), 6);
        assert_eq!(trailing_zeros_16(0x43), 0);
        assert_eq!(trailing_zeros_16(0xE0), 5);
        assert_eq!(trailing_zeros_16(0xF0), 4);
    }
}
