pub fn solve() {
    min_cost_climbing_stairs(vec![10, 15, 20]);
}

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let len = cost.len();
    if len < 2 {
        return 0;
    }
    let (prev1, prev2) = cost.iter().fold((0, 0), |(prev1, prev2), &val| {
        let curr = val + prev1.min(prev2);
        (curr, prev1)
    });
    prev1.min(prev2)
}
