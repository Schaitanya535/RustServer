#[derive(Debug, PartialEq, Eq, Clone)]
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
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub fn solve() {
    sorted_array_to_bst(vec![1, 2, 3]);
}

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    driver(&nums, 0, nums.len() as i32)
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) {
    match root {
        Some(root) => {
            let borrowed_root = root.borrow();
            inorder_traversal(borrowed_root.left.clone());
            println!("{}", borrowed_root.val);
            inorder_traversal(borrowed_root.right.clone());
        }
        None => {}
    }
}

fn driver(nums: &Vec<i32>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if low == high {
        return None;
    }
    let mid = (low + high) / 2;
    // println!("{}", mid);
    let mut head = TreeNode::new(nums[mid as usize]);
    head.left = driver(nums, low, mid);
    head.right = driver(nums, mid + 1, high);
    Some(Rc::new(RefCell::new(head)))
}

fn unique_occurrences(arr: Vec<i32>) -> bool {
    let (mut map, mut set) = (HashMap::new(), HashSet::new());
    arr.into_iter().for_each(|x| {
        map.entry(x).and_modify(|x| *x += 1).or_insert(1);
    });
    map.values().all(|x| set.insert(*x))
}
