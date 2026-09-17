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
        fn in_order(node: &Rc<RefCell<TreeNode>>, prev: &mut Option<i32>, min: &mut i32){
            let node = node.borrow();
            if let Some(left) = node.left.as_ref() {
                in_order(left, prev, min);
            }
            if let Some(p) = *prev {
                *min = (*min).min(node.val - p);
            }
            *prev = Some(node.val);
            if let Some(right) = node.right.as_ref() {
                in_order(right, prev, min);
            }
        }
        let mut min = i32::MAX;
        let mut prev = None;
        in_order(root.as_ref().unwrap(), &mut prev, &mut min);
        min
    }
}
