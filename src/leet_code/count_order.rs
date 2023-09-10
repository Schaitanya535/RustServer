pub fn solve() {
    let ans = count_orders(3);
    println!("orders count {}", ans);
}

pub fn count_orders(n: i32) -> i32 {
    const IE7: i64 = 1000_000_000 + 7;
    (1..=n).fold(1 as i64, |acc: i64, i| {
        ((acc * (i as i64)) * ((2 * i - 1) as i64)) % IE7
    }) as i32
}
