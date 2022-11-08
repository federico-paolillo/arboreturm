use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type Link<V> = Rc<RefCell<Node<V>>>;
type MaybeLink<V> = Option<Link<V>>;

type WeakLink<V> = Weak<RefCell<Node<V>>>;
type MaybeWeakLink<V> = Option<WeakLink<V>>;

struct Node<V: PartialOrd> {
    parent: MaybeWeakLink<V>,
    left: MaybeLink<V>,
    right: MaybeLink<V>,
    value: V,
}

impl<V: PartialOrd> Node<V> {
    fn root_link(value: V) -> Link<V> {
        Rc::new(RefCell::new(Self {
            parent: None,
            left: None,
            right: None,
            value,
        }))
    }

    fn link(parent: WeakLink<V>, value: V) -> Link<V> {
        Rc::new(RefCell::new(Self {
            parent: Some(parent),
            left: None,
            right: None,
            value,
        }))
    }
}

/// A binary search tree.
///
/// Assuming I've implemented it correctly it will:
/// - Search in `O(log n)`
/// - Insert in `O(log n)`
/// - Delete in `O(log n)`
///
/// It will use O(n) of space.
///
/// [See Wikipedia](https://en.wikipedia.org/wiki/Binary_search_tree) for further details.
///
/// # Examples
///
/// ## Search Binary Search Tree items
///
/// ```
/// let bst = &mut arboretum::Bst::empty();
///
/// bst.insert(10);
/// bst.insert(20);
/// bst.insert(30);
///
/// assert_eq!(true, bst.contains(10));
/// ```
pub struct Bst<V: PartialOrd> {
    root: MaybeLink<V>,
}

impl<V: PartialOrd> Bst<V> {
    pub fn empty() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: V) {
        match self.root {
            Some(ref root) => deep_insert(root, value),
            None => self.root = Some(Node::root_link(value)),
        }
    }

    pub fn contains(&self, value: V) -> bool {
        match self.root {
            None => false,
            Some(ref root) => deep_contains(root, value),
        }
    }
}

fn deep_insert<V: PartialOrd>(node: &Link<V>, value: V) {
    let mut mut_node = node.borrow_mut();

    if mut_node.value <= value {
        if let Some(ref left) = mut_node.left {
            deep_insert(left, value);
        } else {
            mut_node.left = Some(Node::link(Rc::downgrade(node), value));
        }
    } else {
        if let Some(ref right) = mut_node.right {
            deep_insert(right, value);
        } else {
            mut_node.right = Some(Node::link(Rc::downgrade(node), value))
        }
    }
}

fn deep_find<V: PartialOrd>(node: &Link<V>, value: V) -> MaybeWeakLink<V> {
    let ref_node = node.borrow();

    if ref_node.value == value {
        return Some(Rc::downgrade(node));
    }

    if ref_node.value < value {
        match ref_node.left {
            None => None,
            Some(ref left) => deep_find(left, value),
        }
    } else {
        match ref_node.right {
            None => None,
            Some(ref right) => deep_find(right, value),
        }
    }
}

fn deep_contains<V: PartialOrd>(node: &Link<V>, value: V) -> bool {
    let maybe_node = deep_find(node, value);

    match maybe_node {
        None => false,
        Some(_) => true,
    }
}
