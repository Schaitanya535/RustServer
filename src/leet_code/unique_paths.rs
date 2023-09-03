pub fn solve() {
    unique_paths(23, 12);
    // println!("unique paths : {}", ans);
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
    //using formula is C(m + n - 2, n-1)
    (1..n.min(m) as u64).fold(1, |acc: u64, val: u64| {
        acc * ((m + n) as u64 - 2 - val + 1) / val
    }) as i32
}
