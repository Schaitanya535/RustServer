pub mod best_closing_time;
pub mod is_palindrome;
pub mod two_sum;

pub fn leet_code() {
    use best_closing_time::best_closing_time;
    use is_palindrome::is_palindrome;
    use two_sum::two_sum;
    two_sum();
    is_palindrome();
    best_closing_time();
}
