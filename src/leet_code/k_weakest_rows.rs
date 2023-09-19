use std::cmp::Ordering;

pub fn solve() {
    let matrix = vec![vec![1, 1, 0], vec![0, 0, 0], vec![1, 0, 0]];
    let ans = k_weakest_rows(matrix, 2);
    println!("ans is : {:?}", ans);
}

pub fn k_weakest_rows(matrix: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut list: Vec<(i32, i32)> = matrix
        .iter()
        .enumerate()
        .map(|(i, row)| (i as i32, row.iter().sum()))
        .collect();

    list.sort_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        other => other,
    });

    list.iter().map(|x| x.0).take(k as usize).collect()
}
