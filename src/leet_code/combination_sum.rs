use std::collections::HashMap;

pub fn solve() {
    combination_sum4(vec![1, 2, 3], 4);
    // println!("Ans for Combination Sum is {}", ans);
}

fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut memo = HashMap::new();
    let min = nums.iter().min().unwrap();
    helper(&nums, target, &mut memo, *min)
}

fn helper(nums: &Vec<i32>, target: i32, memo: &mut HashMap<i32, i32>, min: i32) -> i32 {
    if let Some(&ans) = memo.get(&target) {
        return ans;
    }
    if target == 0 {
        return 1;
    }
    if target < min {
        return 0;
    }
    let ans = nums
        .iter()
        .filter(|&&x| target >= x)
        .map(|x| helper(&nums, target - x, memo, min))
        .sum();
    memo.insert(target, ans);
    ans
}
