pub fn is_palindrome() {
    let x = 121;
    solve(x);
}

fn solve(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let num_str = x.to_string();
    let num_str_rev = num_str.chars().rev().collect::<String>();
    num_str == num_str_rev
    // let mut num = x;
    // let mut rev = 0;
    // while num != 0 {
    //     let pop = num % 10;
    //     num /= 10;
    //     rev = rev * 10 + pop;
    // }
    // x == rev
}
