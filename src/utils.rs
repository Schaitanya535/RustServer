pub fn combinations(n: i64, r: i64) -> i64 {
    (1..r.min(n - r)).fold(1, |acc, val| (acc * (n - val) / val))
}

pub fn gcd(a: i32, b: i32) -> i32 {
    let reminder = a.max(b) % b.min(a);
    if reminder == 0 {
        return b.min(a);
    }
    return gcd(reminder, b.min(a));
}
