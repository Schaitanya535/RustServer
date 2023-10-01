pub fn solve() {
    let ans = sort_array_by_parity(vec![2, 3, 4, 5]);
    let words = reverse_words("Let's take LeetCode contest".to_owned());
    println!("ans : {:?}", ans);
    println!("words : {:?}", words);
}

pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut even = Vec::new();
    let mut odd = Vec::new();

    for num in nums {
        if num % 2 == 0 {
            even.push(num);
        } else {
            odd.push(num);
        }
    }

    even.extend(odd);
    even
}

pub fn reverse_words(s: String) -> String {
    s.split_whitespace()
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
