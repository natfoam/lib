mod collect;
mod empty;
mod flat_scan;
mod fold;
mod iter;
mod list;
mod option;
mod take;

pub use collect::*;
pub use empty::*;
pub use flat_scan::*;
pub use fold::*;
pub use iter::*;
pub use list::*;
pub use option::*;
pub use take::*;

#[cfg(test)]
mod tests {
    use super::*;

    struct Ref<'a>(&'a mut u32);

    impl<'a> ListFn for Ref<'a> {
        type Item = u32;
        type End = Self;
        fn list(self) -> List<Self> {
            let first = *self.0;
            if first < 10 {
                *self.0 += 1;
                List::Some(first, self)
            } else {
                List::End(self)
            }
        }
    }

    struct Range10(u32);

    impl<'a> ListFn for Range10 {
        type Item = u32;
        type End = Self;
        fn list(self) -> List<Self> {
            let first = self.0;
            if first < 10 {
                List::Some(first, Self(first + 1))
            } else {
                List::End(self)
            }
        }
    }

    #[test]
    fn as_ref() {
        let mut i = 0;
        let v = Ref(&mut i).iter().collect::<Vec<u32>>();
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn as_range() {
        let v = Range10(0).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn option() {
        {
            let a = Some(5);
            let v = a.iter().collect::<Vec<u32>>();
            assert_eq!(v, vec![5]);
        }
        {
            let a = None;
            let v = a.iter().collect::<Vec<u32>>();
            assert_eq!(v, Vec::new());
        }
    }

    #[test]
    fn foreach() {
        let mut v = Vec::new();
        for elem in Range10(2).iter() {
            v.push(elem);
        }
        assert_eq!(v, vec![2, 3, 4, 5, 6, 7, 8, 9])
    }
}
