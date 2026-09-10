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
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn search_subtree(root: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            let Some(root) = root else { return (0, 0, 0) };
            let root = root.borrow();

            let (ls, lc, lr) = search_subtree(root.left.as_ref());
            let (rs, rc, rr) = search_subtree(root.right.as_ref());
            let sub_tree_sum = ls + rs + root.val;
            let sub_tree_count = lc + rc + 1;
            let count = lr + rr + i32::from(sub_tree_sum / sub_tree_count == root.val);

            (sub_tree_sum, sub_tree_count, count)
        }
        search_subtree(root.as_ref()).2
    }
}
