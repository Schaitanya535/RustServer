use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve() {
    let path = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
    let ans = minimum_effort_path(path);
    println!("ans is : {}", ans);
}

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let rows = heights.len();
    let cols = heights[0].len();
    let mut dist = vec![vec![i32::MAX; cols]; rows];
    let mut min_heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

    dist[0][0] = 0;
    min_heap.push(Reverse((0, 0, 0)));

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(Reverse((effort, x, y))) = min_heap.pop() {
        if effort > dist[x][y] {
            continue;
        }
        if x == rows - 1 && y == cols - 1 {
            return effort;
        }
        for (dx, dy) in directions.iter() {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx < rows && ny < cols {
                let new_effort = std::cmp::max(effort, (heights[x][y] - heights[nx][ny]).abs());
                if new_effort < dist[nx][ny] {
                    dist[nx][ny] = new_effort;
                    min_heap.push(Reverse((new_effort, nx, ny)));
                }
            }
        }
    }
    -1
}
