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
        let root = root.as_ref().unwrap().borrow();
        let (left, right) = match (&root.left, &root.right) {
            (None, None) => return true,
            (Some(l), Some(r)) => (l, r),
            _ => return false,
        };

        fn check(left: &Rc<RefCell<TreeNode>>, right: &Rc<RefCell<TreeNode>>) -> bool {
            let (l, r) = (left.borrow(), right.borrow());
            if l.val != r.val {
                return false;
            }

            let check_out = match (l.left.as_ref(), r.right.as_ref()) {
                (None, None) => true,
                (Some(ll), Some(rr)) => check(ll, rr),
                _ => false,
            };
            
            let check_in = match (l.right.as_ref(), r.left.as_ref()) {
                (None, None) => true,
                (Some(lr), Some(rl)) => check(lr, rl),
                _ => false,
            };

            check_out && check_in
        }

        check(left, right)
    }
}

