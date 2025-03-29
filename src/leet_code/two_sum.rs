use std::{collections::HashMap, iter::zip, slice::Windows};

pub fn solve() -> Vec<i32> {
    two_sum(vec![2, 7, 11, 15], 9)
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        let complement = target - nums[i];
        if map.contains_key(&complement) {
            result.push(map.get(&complement).unwrap().clone());
            result.push(i as i32);
            return result;
        } else {
            map.insert(nums[i], i as i32);
        }
    }
    result
}

fn score_of_string(s: String) -> i32 {
    /* zip(s.chars(), s.chars().skip(1))
    .map(|(a, b)| (a as i32 - b as i32).abs())
    .sum() */

    s.as_bytes()
        .windows(2)
        .map(|w| (w[0] as i32 - w[1] as i32).abs())
        .sum()
}
