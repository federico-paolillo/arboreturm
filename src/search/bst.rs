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

    pub fn remove(&mut self, value: V) {
        match self.root {
            None => (),
            Some(_) => deep_delete(self, value),
        };
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

fn deep_find<V: PartialOrd>(node: &Link<V>, value: V) -> MaybeLink<V> {
    let ref_node = node.borrow();

    if ref_node.value == value {
        return Some(Rc::clone(node));
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

fn deep_delete<V: PartialOrd>(bst: &mut Bst<V>, value: V) {}

fn shift_nodes<V: PartialOrd>(bst: &mut Bst<V>, node_a: &mut Link<V>, o_node_b: &mut MaybeLink<V>) {
    let m_node_a = node_a.borrow_mut();

    if let None = m_node_a.parent {
        match o_node_b {
            None => bst.root.take(),
            Some(ref r_node_b) => bst.root.replace(Rc::clone(r_node_b)),
        };

        return;
    }

    let w_node_a_parent = m_node_a.parent.as_ref().unwrap();
    let o_node_a_parent = w_node_a_parent.upgrade();

    if let None = o_node_a_parent {
        panic!("Weird Bst structure detected. Found an orphan node");
    }

    let node_a_parent = o_node_a_parent.unwrap();
    let mut m_node_a_parent = node_a_parent.borrow_mut();

    if let Some(ref r_node_a_parent_left) = m_node_a_parent.left {
        // Node A is at the left of Node A Parent
        if Rc::ptr_eq(node_a, r_node_a_parent_left) {
            // We replace Node A on Node A Parent left with Node B
            match o_node_b {
                None => m_node_a_parent.left.take(),
                Some(ref r_node_b) => m_node_a_parent.left.replace(Rc::clone(r_node_b)),
            };

            return;
        }
    }
    // Node A is at the right of Node A Parent
    else {
        // We replace Node A on Node A Parent right with Node B
        match o_node_b {
            None => m_node_a_parent.right.take(),
            Some(ref r_node_b) => m_node_a_parent.right.replace(Rc::clone(r_node_b)),
        };

        return;
    }

    // Now we fix Node B by changing Node B Parent to Node A Parent
    if let Some(ref r_node_b) = o_node_b {
        let mut m_node_b = r_node_b.borrow_mut();
        m_node_b.parent.replace(Rc::downgrade(&node_a_parent));
    }
}
