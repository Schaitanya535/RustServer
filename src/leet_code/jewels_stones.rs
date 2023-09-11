pub fn solve() {
    num_jewels_in_stones("A".to_owned(), "aAAbbbb".to_owned());
}

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    jewels
        .chars()
        .map(|c| stones.chars().filter(|s| s.eq(&c)).count())
        .sum::<usize>() as i32
}

pub fn num_jewels_in_stones2(jewels: String, stones: String) -> i32 {
    stones
        .chars()
        .filter(|&stone| jewels.contains(stone))
        .count() as i32
}
