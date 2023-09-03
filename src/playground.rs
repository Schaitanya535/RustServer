pub fn trying_flatten() {
    let vector = [vec![1, 3], vec![1, 2]];
    let ans: Vec<i32> = vector.into_iter().flatten().collect();
    for i in ans {
        println!("{}", i);
    }
}
