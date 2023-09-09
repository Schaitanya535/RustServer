pub fn solve() {
    let ans = generate(5);
    println!("pascal triangle is:  {:?}", ans);
}

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    ans.push(vec![1]);
    if num_rows == 1 {
        return ans;
    }
    for _ in 1..num_rows {
        let prev = ans.last().unwrap();
        let mut curr = vec![1];
        for j in 1..prev.len() {
            let val = prev[j] + prev[j - 1];
            curr.push(val);
        }
        curr.push(1);
        ans.push(curr);
    }
    return ans;
}
