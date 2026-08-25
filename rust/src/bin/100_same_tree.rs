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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {

        fn check (p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(p_node), Some(q_node)) => {
                    let (p_ref, q_ref) = (p_node.borrow(), q_node.borrow());
                    p_ref.val == q_ref.val 
                        && check(p_ref.left.as_ref(), q_ref.left.as_ref())
                        && check(p_ref.right.as_ref(), q_ref.right.as_ref())
                },
                _ => false
            }
        }

        check(p.as_ref(), q.as_ref())
    }
}

struct Solution {}
