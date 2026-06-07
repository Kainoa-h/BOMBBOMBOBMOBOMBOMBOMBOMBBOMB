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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::<i32, Rc<RefCell<TreeNode>>>::new();
        for e_node in descriptions {
            let (node_value, child_value, is_left) = (e_node[0], e_node[1], e_node[2] == 1);
            let tree_node = map
                .entry(node_value)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(node_value))))
                .clone();
            let child_node = map
                .entry(child_value)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child_value))))
                .clone();
            if is_left {
                tree_node.borrow_mut().left = Some(child_node.clone());
            } else {
                tree_node.borrow_mut().right = Some(child_node.clone());
            }
        }

        for node in map.values() {
            let c = Rc::strong_count(node);
            if c == 1 {
                return Some(node.clone());
            }
        }

        None
    }
}
struct Solution {}

fn main() {
    let x = Solution::create_binary_tree(vec![
        vec![20, 15, 1],
        vec![20, 17, 0],
        vec![50, 20, 1],
        vec![50, 80, 0],
        vec![80, 19, 1],
    ]);
    println!("{:?}", x);
}
