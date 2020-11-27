use super::Node;
use std::collections::VecDeque;
use std::marker::PhantomData;

/// Level order traverse iterator.
#[derive(Debug)]
pub struct LevelOrderIter<'a, T> {
    last: *const Node<T>,
    queue: VecDeque<*const Node<T>>,
    marker: PhantomData<&'a Node<T>>,
    level: usize,
}

impl<'a, T> LevelOrderIter<'a, T> {
    /// Create a level order traverse iter.
    pub fn new(node: &'a Node<T>) -> Self {
        let ptr = node as *const _;
        let mut queue = VecDeque::new();
        queue.push_back(ptr);
        Self {
            last: ptr,
            queue,
            level: 0,
            marker: PhantomData::default(),
        }
    }

    /// Return the level in the tree of the next item
    /// returned by `next`.
    pub fn level(&self) -> usize {
        self.level
    }
}

impl<'a, T> Iterator for LevelOrderIter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ptr) = self.queue.pop_front() {
            unsafe {
                if let Some(node) = ptr.as_ref() {
                    if let Some(left) = node.left() {
                        self.queue.push_back(left as *const _);
                    }
                    if let Some(right) = node.right() {
                        self.queue.push_back(right as *const _);
                    }

                    let level = self.level;

                    // update the last pointer.
                    if self.last == ptr {
                        if let Some(last) = self.queue.back() {
                            self.last = *last;
                        }
                        self.level += 1;
                    }
                    Some((level, node.data()))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
