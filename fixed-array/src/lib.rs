use core::ops::IndexMut;

pub trait Array: Sized + IndexMut<usize>
where
    Self::Output: Sized,
{
    type Item;
    const SIZE: usize;
}

impl<Item, const SIZE: usize> Array for [Item; SIZE] {
    type Item = Item;
    const SIZE: usize = SIZE;
}
