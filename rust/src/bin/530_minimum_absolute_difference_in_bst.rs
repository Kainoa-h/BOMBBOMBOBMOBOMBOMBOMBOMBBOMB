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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn in_order(node: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
            let node = node.borrow();
            let mut v = Vec::new();
            if let Some(left) = node.left.as_ref() {
                v.append(&mut in_order(left));
            }
            v.push(node.val);
            if let Some(right) = node.right.as_ref() {
                v.append(&mut in_order(right));
            }
            v
        }
        let vec = in_order(root.as_ref().unwrap());

        vec.windows(2).map(|x| x[1] - x[0]).min().unwrap_or(0)
    }
}
