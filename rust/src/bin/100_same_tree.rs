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
        let mut p_stack = vec![p];
        let mut q_stack = vec![q];

        while p_stack.len() > 0 || q_stack.len() > 0 {
            let (Some(po), Some(qo)) = (p_stack.pop(), q_stack.pop()) else {
                return false;
            };
            if po.is_some() != qo.is_some() {
                return false;
            }
            if let (Some(p), Some(q)) = (po, qo) {
                if p.borrow().val != q.borrow().val {
                    return false;
                }
                p_stack.push(p.borrow().left.clone());
                p_stack.push(p.borrow().right.clone());
                q_stack.push(q.borrow().left.clone());
                q_stack.push(q.borrow().right.clone());
            }
        }

        true
    }
}

struct Solution {}
