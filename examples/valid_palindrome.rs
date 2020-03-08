// https://leetcode.com/problems/valid-palindrome/
struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s
            .chars()
            .map(|mut c| {
                c.make_ascii_lowercase();
                c
            })
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<Vec<char>>();

        if chars.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            if chars[left] != chars[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}

fn main() {
    assert_eq!(
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
        true
    );
    assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    assert_eq!(Solution::is_palindrome("0P".to_string()), false);
}
