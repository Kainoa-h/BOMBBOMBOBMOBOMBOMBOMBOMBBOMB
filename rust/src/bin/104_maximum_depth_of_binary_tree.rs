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
      right: None
    }
  }
}

use std::cmp;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn search(node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32{
            let Some(node) = node else { return depth - 1;};
            let node = node.borrow();
            cmp::max(search(node.left.clone(), depth + 1), search(node.right.clone(), depth + 1))
        }
        search(root, 1)
    }

}

struct Solution {}
