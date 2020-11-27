use crate::{Error, Result};
use std::collections::VecDeque;
use std::fmt;

/// Binary tree iter.
pub mod iter;

type Link<T> = Option<BoxedNode<T>>;
type BoxedNode<T> = Box<Node<T>>;

/// Binary tree node.
#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    /// Create a node with no links.
    pub fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    /// Convert into a boxed node.
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Create a builder.
    pub fn builder() -> NodeBuilder<T> {
        NodeBuilder::default()
    }

    /// Get the ref of left child.
    pub fn left(&self) -> Option<&Node<T>> {
        self.left.as_deref()
    }

    /// Get the ref of right child.
    pub fn right(&self) -> Option<&Node<T>> {
        self.right.as_deref()
    }

    /// Get the ref of the containing data.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Create a level order traverse iterator
    /// use this node as root.
    pub fn level_order_iter(&self) -> iter::LevelOrderIter<T> {
        iter::LevelOrderIter::new(self)
    }
}

impl<T> Node<T> {
    /// Pre order map.
    pub fn pre_order_map<U, F>(self, mut f: F) -> Node<U>
    where
        F: FnMut(T) -> U,
    {
        self.pre_order_map_inner(&mut f)
    }

    fn pre_order_map_inner<U, F>(self, f: &mut F) -> Node<U>
    where
        F: FnMut(T) -> U,
    {
        Node {
            data: f(self.data),
            left: self.left.map(|node| node.pre_order_map_inner(f).boxed()),
            right: self.right.map(|node| node.pre_order_map_inner(f).boxed()),
        }
    }
}

impl<T> Node<T> {
    /// Mid order map.
    pub fn mid_order_map<U, F>(self, mut f: F) -> Node<U>
    where
        F: FnMut(T) -> U,
    {
        self.mid_order_map_inner(&mut f)
    }

    fn mid_order_map_inner<U, F>(self, f: &mut F) -> Node<U>
    where
        F: FnMut(T) -> U,
    {
        Node {
            left: self.left.map(|node| node.mid_order_map_inner(f).boxed()),
            data: f(self.data),
            right: self.right.map(|node| node.mid_order_map_inner(f).boxed()),
        }
    }
}

impl<T> Node<T> {
    /// Post order map.
    pub fn post_order_map<U, F>(self, mut f: F) -> Node<U>
    where
        F: FnMut(T) -> U,
    {
        self.post_order_map_inner(&mut f)
    }

    fn post_order_map_inner<U, F>(self, f: &mut F) -> Node<U>
    where
        F: FnMut(T) -> U,
    {
        Node {
            left: self.left.map(|node| node.post_order_map_inner(f).boxed()),
            right: self.right.map(|node| node.post_order_map_inner(f).boxed()),
            data: f(self.data),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = self as *const _;
        let mut last = ptr;
        let mut back = ptr;
        let mut max_width = 1;
        let mut queue: VecDeque<Option<*const Self>> = VecDeque::new();
        let mut levels = Vec::new();
        let mut level = Vec::new();
        queue.push_back(Some(ptr));

        while let Some(ptr) = queue.pop_front() {
            if let Some(ptr) = ptr {
                unsafe {
                    if let Some(node) = ptr.as_ref() {
                        queue.push_back(node.left().map(|node| {
                            let ptr = node as *const _;
                            back = ptr;
                            ptr
                        }));
                        queue.push_back(node.right().map(|node| {
                            let ptr = node as *const _;
                            back = ptr;
                            ptr
                        }));
                        let c = format!("{}", node.data);
                        if c.len() > max_width {
                            max_width = c.len();
                        }
                        level.push(c);
                        if last == ptr {
                            last = back;
                            levels.push(level);
                            level = Vec::new();
                        }
                    }
                }
            } else {
                level.push(" ".to_owned());
            }
        }
        let len = levels.len();
        let blank = " ".repeat(max_width);
        for (idx, level) in levels.iter().enumerate() {
            let left = (1 << (len - idx - 1)) - 1;
            let mid = (1 << (len - idx)) - 1;
            if idx > 0 {
                for i in (0..(1 << (len - idx - 1))).rev() {
                    let left_space = blank.repeat(left + i);
                    let mid_space = blank.repeat(mid - 2 * i);
                    let right_space = blank.repeat(mid + 2 * i);
                    write!(f, "{}", left_space)?;
                    for (idx, c) in level.iter().enumerate() {
                        let branch = if idx % 2 == 0 {
                            let tag = if c != " " { "/" } else { " " };
                            format!("{:^width$}", tag, width = max_width)
                        } else {
                            let tag = if c != " " { r"\" } else { " " };
                            format!(
                                "{}{:^width$}{}",
                                mid_space,
                                tag,
                                right_space,
                                width = max_width
                            )
                        };
                        write!(f, "{}", branch)?;
                    }
                    writeln!(f)?;
                }
            }

            let left_space = blank.repeat(left);
            let space = blank.repeat(mid);
            write!(f, "{}", left_space)?;
            for c in level.iter() {
                write!(f, "{:^width$}{}", c, space, width = max_width)?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

/// Binary tree node builder.
#[derive(Debug, Clone)]
pub struct NodeBuilder<T> {
    data: Option<T>,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Default for NodeBuilder<T> {
    fn default() -> Self {
        Self {
            data: None,
            left: None,
            right: None,
        }
    }
}

impl<T> NodeBuilder<T> {
    /// Build the node.
    /// # Errors
    /// Return `MissingDataField` Error when the data field is not set.
    pub fn build(self) -> Result<Node<T>> {
        if let Some(data) = self.data {
            Ok(Node {
                data,
                left: self.left,
                right: self.right,
            })
        } else {
            Err(Error::MissingDataField)
        }
    }

    /// Set `data` field.
    pub fn data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    /// Set `left` field.
    pub fn left(mut self, node: Node<T>) -> Self {
        self.left = Some(node.boxed());
        self
    }

    /// Set `right` field.
    pub fn right(mut self, node: Node<T>) -> Self {
        self.right = Some(node.boxed());
        self
    }
}
