use super::*;

pub trait Collection {
    type Item;
    fn add(self, item: Self::Item) -> Self; 
}

impl<T> Collection for Vec<T> {
    type Item = T;

    fn add(mut self, item: Self::Item) -> Self {
        self.push(item);
        self
    }
}