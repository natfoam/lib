use super::*;
use take_mut::take;

/// Any `&mut Iterator` is a `ListFn<End = Self>`.
impl<I: Iterator> ListFn for &mut I {
    type Item = I::Item;
    type End = Self;
    /// Converts an iterator into a list.
    fn next(self) -> ListState<Self> {
        match self.next() {
            Option::None => ListState::End(self),
            Option::Some(first) => ListState::some(first, self ),
        }
    }
}

impl<I: Iterator> ResultFn for &mut I {
    type Result = ();
    fn result(self) {}
}

pub struct ListIterator<S: ListFn<End = S>>(S);

impl<S: ListFn<End = S>> Iterator for ListIterator<S> {
    type Item = S::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        take(&mut self.0, |list| match list.next() {
            ListState::End(end) => end,
            ListState::Some(some) => {
                result = Some(some.first);
                some.next
            }
        });
        result
    }
}

/// Note: we can't use the standard std::iter::IntoIterator because it has
/// a conflicting implementation.
pub trait Iter: ListFn<End = Self> {
    /// Converts a list to an iterator.
    fn iter(self) -> ListIterator<Self> {
        ListIterator(self)
    }
}

impl<S: ListFn<End = Self>> Iter for S {}

pub enum ListIteratorWrap<L: ListFn> {
    List(L),
    End(L::End),
}

impl<L: ListFn> Iterator for ListIteratorWrap<L> {
    type Item = L::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        take(self, |wrap| match wrap {
            ListIteratorWrap::List(list) => match list.next() {
                ListState::End(end) => ListIteratorWrap::End(end),
                ListState::Some(some) => {
                    result = Some(some.first);
                    ListIteratorWrap::List(some.next)
                }
            },
            end => end,
        });
        result
    }
}

pub trait IterWrap: ListFn {
    fn iter_wrap(self) -> ListIteratorWrap<Self> {
        ListIteratorWrap::List(self)
    }
}

impl<L: ListFn> IterWrap for L {}
