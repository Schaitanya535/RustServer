fn count_bits(n: i32) -> Vec<i32> {
    let mut ans = vec![0; (n + 1) as usize];
    for i in 1..=n {
        ans[i as usize] = ans[(i >> 1) as usize] + (i & 1);
    }
    return ans;
}

pub fn all_ones(nums: Vec<i32>) -> bool {
    nums.iter().all(|&x| x == 1)
}

pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    let mut ans = 0;

    if nums.len() < 3 {
        if all_ones(nums) {
            return 0;
        } else {
            return -1;
        }
    }

    for i in 0..nums.len() - 2 {
        if nums[i] == 0 {
            nums[i] = 1;
            nums[i + 1] = 1 - nums[i + 1];
            nums[i + 2] = 1 - nums[i + 2];
            ans += 1;
        }
    }

    if all_ones(nums) {
        return ans;
    } else {
        return -1;
    }
}

pub fn solve() {
    count_bits(30);
    // ans.iter().for_each(|i| print!("{}", i));
}
