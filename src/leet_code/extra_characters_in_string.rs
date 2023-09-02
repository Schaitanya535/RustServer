// You are given a 0-indexed string s and a dictionary of words dictionary. You have to break s into one or more non-overlapping substrings such that each substring is present in dictionary. There may be some extra characters in s which are not present in any of the substrings.

// Return the minimum number of extra characters left over if you break up s optimally.

// Example 1:

// Input: s = "leet_s_code", dictionary = ["leet","code","leetcode"]
// Output: 1
// Explanation: We can break s in two substrings: "leet" from index 0 to 3 and "code" from index 5 to 8. There is only 1 unused character (at index 4), so we return 1.

// Example 2:

// Input: s = "say_hello_world", dictionary = ["hello","world"]
// Output: 3
// Explanation: We can break s in two substrings: "hello" from index 3 to 7 and "world" from index 8 to 12. The characters at indices 0, 1, 2 are not used in any substring and thus are considered as extra characters. Hence, we return 3.

use std::collections::HashSet;

pub fn solve() {
    let dictionary = vec![
        "leet".to_owned(),
        "code".to_owned(),
        "say".to_owned(),
        "world".to_owned(),
    ];
    min_extra_char("hello_world".to_owned(), dictionary);
}

fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    let max_val = s.len() as i32 + 1;
    let mut dp = vec![max_val; s.len() + 1];
    dp[0] = 0;

    let dictionary_set: HashSet<_> = dictionary.into_iter().collect();

    for i in 1..=s.len() {
        dp[i] = dp[i - 1] + 1;

        for l in 1..=i {
            if dictionary_set.contains(&s[i - l..i].to_string()) {
                dp[i] = std::cmp::min(dp[i], dp[i - l]);
            }
        }
    }
    dp[s.len()] as i32
}
