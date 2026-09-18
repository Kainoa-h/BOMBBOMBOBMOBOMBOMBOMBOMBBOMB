// Definition for a binary tree node.
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

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check(
            left: Option<&Rc<RefCell<TreeNode>>>,
            right: Option<&Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                (None, None) => true,
                (None, Some(_)) | (Some(_), None) => false,
                (Some(l), Some(r)) => {
                    let (l, r) = (l.borrow(), r.borrow());
                    l.val == r.val
                        && check(l.left.as_ref(), r.right.as_ref())
                        && check(l.right.as_ref(), r.left.as_ref())
                }
            }
        }
        
        match root {
            None => true,
            Some(r) => check(r.borrow().left.as_ref(), r.borrow().right.as_ref())
        }
    }
}
