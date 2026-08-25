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
        let mut stack = vec![(p,q)];

        while let Some((po,qo)) = stack.pop() {
            match (po,qo) {
                (None, None) => continue,
                (Some(p), Some(q)) => {
                    let (p, q) = (p.borrow(), q.borrow());
                    if p.val != q.val {
                        return false;
                    }
                    stack.push((p.left.clone(), q.left.clone()));
                    stack.push((p.right.clone(), q.right.clone()));
                }
                _ => return false
            }
        }

        true
    }
}

struct Solution {}
