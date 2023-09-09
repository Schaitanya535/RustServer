pub fn solve() {
    let ans = shuffle(vec![1, 2, 3, 4], 2);
    println!("shuffled array is {:?}", ans);
}

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    (0..n).fold(vec![], |mut a, i| {
        a.append(&mut vec![nums[i], nums[i + n]]);
        a
    })
}
