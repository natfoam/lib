use std::ops::IndexMut;

pub trait Array: IndexMut<usize>
where
    Self::Output: Sized,
{
    const SIZE: usize;
}

impl<Item, const SIZE: usize> Array for [Item; SIZE] {
    const SIZE: usize = SIZE;
}