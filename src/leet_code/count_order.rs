use crate::utils::{factorial, pow_mod, MOD};

pub fn solve() {
    count_orders(3);
    count_orders_math(3);
    // println!("orders count {}", ans);
    // println!("orders count {}", ans2);
}

pub fn count_orders(n: i32) -> i32 {
    (1..=n).fold(1 as i64, |acc: i64, i| {
        ((acc * (i as i64)) * ((2 * i - 1) as i64)) % MOD
    }) as i32
}

pub fn count_orders_math(n: i32) -> i32 {
    //* Total arrangements are 2n!
    //* Out of which number of ways to arrange a set of PickUp and delivery are 2^n
    //* So, the total arrangements come out to be 2n!/2^n

    let two_n_factorial = factorial(2 * n);
    let power = pow_mod(2, n);
    // println!("power : {} and factorial: {}", power, two_n_factorial);
    ((two_n_factorial as i64 / power as i64) % MOD) as i32
}
