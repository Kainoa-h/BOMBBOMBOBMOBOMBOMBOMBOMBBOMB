use std::cell::{RefCell, RefMut};
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
        fn swap_children(node: &mut RefMut<'_, TreeNode>) {
            let left = node.left.take();
            node.left = node.right.take();
            node.right = left;

            if let Some(left) = &node.left {
                swap_children(&mut left.borrow_mut());
            }
            if let Some(right) = &node.right {
                swap_children(&mut right.borrow_mut());
            }
        }
        if let Some(r) = &root {
            swap_children(&mut r.borrow_mut());
        }

        root
    }
}

struct Solution{}
