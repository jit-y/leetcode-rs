// https://leetcode.com/problems/length-of-last-word/
struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.split_ascii_whitespace();
        let mut result = 0;

        if let Some(last) = s.last() {
            result += last.len() as i32;
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    assert_eq!(Solution::length_of_last_word("Hello World ".to_string()), 5);
}
