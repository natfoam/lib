use super::*;
use take_mut::take;

/// Any `&mut Iterator` is a `ListFn<End = Self>`.
impl<I: Iterator> ListFn for &mut I {
    type Item = I::Item;
    type End = Self;
    /// Converts an iterator into a list.
    fn list(self) -> List<Self> {
        match self.next() {
            Option::None => List::End(self),
            Option::Some(first) => List::Some(first, self),
        }
    }
}

pub struct ListIterator<S: ListFn<End = S>>(S);

impl<S: ListFn<End = S>> Iterator for ListIterator<S> {
    type Item = S::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        take(&mut self.0, |list| match list.list() {
            List::End(end) => end,
            List::Some(first, next) => {
                result = Some(first);
                next
            }
        });
        result
    }
}

/// Note: we can't use the standard std::iter::IntoIterator because it has
/// a conflicting implementation.
pub trait IntoIter: ListFn<End = Self> {
    /// Converts a list to an iterator.
    fn iter(self) -> ListIterator<Self> {
        ListIterator(self)
    }
}

impl<S: ListFn<End = Self>> IntoIter for S {}
