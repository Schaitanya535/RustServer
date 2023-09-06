pub fn solve() {
    // let ans = min_partitions("12346668".to_owned());
    min_partitions_2("12346668".to_owned());
    // println!("min num of partitions {}", ans);
}

pub fn min_partitions(n: String) -> i32 {
    const RADIX: u32 = 10;
    let mut max = '0';
    for i in n.chars() {
        if i.to_digit(RADIX).unwrap() > max.to_digit(RADIX).unwrap() {
            max = i;
        }
    }
    return max.to_digit(RADIX).unwrap() as i32;
}

pub fn min_partitions_2(n: String) -> i32 {
    n.chars().max().unwrap() as i32 - '0' as i32
}
