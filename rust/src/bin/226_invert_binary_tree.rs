use std::cell::{RefCell, RefMut};
use std::mem;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn swap_children(node: Option<&Rc<RefCell<TreeNode>>>) {
            if let Some(n) = node {
                let mut n_borrow = n.borrow_mut();
                let n_ref = &mut *n_borrow;
                mem::swap(&mut n_ref.left, &mut n_ref.right);
                swap_children(n_ref.left.as_ref());
                swap_children(n_ref.right.as_ref());
            }
        }
        swap_children(root.as_ref());

        root
    }
}

struct Solution {}
