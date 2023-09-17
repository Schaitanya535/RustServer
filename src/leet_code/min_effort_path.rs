pub fn solve() {
    let path = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
    let ans = minimum_effort_path(path);
    println!("ans is : {}", ans);
}

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let col = heights.len();
    let mut sol_path = vec![vec![0; col]; col];
    ans
}
