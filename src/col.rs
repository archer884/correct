use std::collections::LinkedList;

pub struct CappedList<T> {
    pub list: LinkedList<T>,
    max_size: usize,
}

impl<T> CappedList<T> {
    pub fn new(max_size: usize) -> CappedList<T> {
        CappedList {
            list: LinkedList::new(),
            max_size: max_size,
        }
    }

    pub fn push_max(&mut self, item: T) {
        if self.list.len() == self.max_size { self.list.pop_back(); }
        self.list.push_front(item);
    }
}
