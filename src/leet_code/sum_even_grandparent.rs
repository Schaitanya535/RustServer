// Definition for a binary tree node.

use std::cell::RefCell;
use std::rc::Rc;
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

fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, grand_parent: i32, parent: i32) -> i32 {
    match root {
        Some(root_ref) => {
            let root_node = root_ref.borrow();
            let mut total = 0;

            if grand_parent % 2 == 0 {
                total += root_node.val;
            }

            total += dfs(&root_node.left, parent, root_node.val);
            total += dfs(&root_node.right, parent, root_node.val);

            total
        }
        None => 0,
    }
}

pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //    let a =  root.as_ref();
    dfs(&root, 1, 1)
}
