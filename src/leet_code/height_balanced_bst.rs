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

pub fn solve() {
    sorted_array_to_bst(vec![1, 2, 3]);
}

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    driver(&nums, 0, nums.len() as i32)
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
