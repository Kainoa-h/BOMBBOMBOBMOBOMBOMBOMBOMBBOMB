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
            let Some(root) = root else {return (0,0,0)};
            let root = root.borrow();

            let mut sub_tree_sum = root.val;
            let mut sub_tree_count = 1;

            let left = search_subtree(root.left.as_ref());
            sub_tree_sum += left.0;
            sub_tree_count += left.1;
            let right = search_subtree(root.right.as_ref());
            sub_tree_sum += right.0;
            sub_tree_count += right.1;

            let count = left.2 + right.2 + i32::from(sub_tree_sum/sub_tree_count == root.val);

            (sub_tree_sum, sub_tree_count, count)
        }
        search_subtree(root.as_ref()).2
    }
}
