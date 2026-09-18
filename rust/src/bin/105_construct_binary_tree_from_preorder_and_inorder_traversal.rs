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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            for (pre_i, &x) in preorder.iter().enumerate() {
                for (in_i, &y) in inorder.iter().enumerate() {
                    if x != y {
                        continue;
                    }
                    let mut node = TreeNode::new(x);
                    node.left = build(&preorder[pre_i + 1..], &inorder[..in_i]);
                    node.right = build(&preorder[pre_i + 1..], &inorder[in_i + 1..]);
                    return Some(Rc::new(RefCell::new(node)))
                }
            }
            None
        }

        build(&preorder, &inorder)
    }
}
