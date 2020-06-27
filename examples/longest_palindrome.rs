struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut result = 0;

        for c in s.chars() {
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        for (_, v) in map {
            result += v / 2 * 2;

            if result % 2 == 0 && v % 2 == 1 {
                result += 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
}
