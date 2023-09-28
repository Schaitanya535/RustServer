use std::collections::{HashMap, HashSet};

pub fn solve() {
    let ans = remove_duplicate_letters("aadbbc".to_owned());
    println!("ans : {}", ans);
}

pub fn remove_duplicate_letters(s: String) -> String {
    let mut stack: Vec<char> = Vec::new();
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    let mut seen: HashSet<char> = HashSet::new();

    for (i, char) in s.char_indices() {
        last_seen.insert(char, i);
    }
    for (i, char) in s.char_indices() {
        if seen.contains(&char) {
            continue;
        }
        while let Some(&top) = stack.last() {
            if (char < top) && (i < *last_seen.get(&top).unwrap()) {
                seen.remove(&stack.pop().unwrap());
            } else {
                break;
            }
        }
        stack.push(char);
        seen.insert(char);
    }
    stack.into_iter().collect()
}
