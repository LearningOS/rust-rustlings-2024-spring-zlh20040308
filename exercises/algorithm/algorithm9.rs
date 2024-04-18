/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        self.items.push(value);
        let mut cur_node_index = self.count;
        let mut cur_node_parent_index = self.parent_idx(cur_node_index);
        while self.valid_index(cur_node_parent_index) {
            if (self.comparator)(&self.items[cur_node_index], &self.items[cur_node_parent_index]){
                self.items.swap(cur_node_index, cur_node_parent_index);
                cur_node_index = cur_node_parent_index;
                cur_node_parent_index = self.parent_idx(cur_node_index);
            }else {
                break;
            }
            
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn valid_index(&self, idx: usize) -> bool {
        idx <= self.count && idx >=1
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if self.valid_index(left) && self.valid_index(right) {
            if (self.comparator)(&self.items[left], &self.items[right]) {
                left
            } else {
                right
            }
        } else {
            left
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count == 0 {
            return None;
        }
        if self.count == 1 {
            self.count -= 1;
            return self.items.pop();
        }
        self.items.swap(1, self.count);
        let res = self.items.pop();
        let mut cur_node_index: usize = 1;
        self.count -= 1;
        while self.children_present(cur_node_index) {
            let smallest = self.smallest_child_idx(cur_node_index);
            if (self.comparator)(&self.items[cur_node_index], &self.items[smallest]) {
                break;
            }else {
                self.items.swap(cur_node_index, smallest);
                cur_node_index = smallest;
            }
        }
        res
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
