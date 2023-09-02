fn count_bits(n: i32) -> Vec<i32> {
    let mut ans = vec![0; (n + 1) as usize];
    for i in 1..=n {
        ans[i as usize] = ans[(i >> 1) as usize] + (i & 1);
    }
    return ans;
}

pub fn solve() {
    count_bits(30);
    // ans.iter().for_each(|i| print!("{}", i));
}
