struct Solution {}
impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }

        if a.is_empty() {
            return true;
        }

        format!("{}{}", a, a).as_str().contains(b.as_str())
    }
}

fn main() {
    assert_eq!(
        Solution::rotate_string("abcde".to_string(), "cdeab".to_string()),
        true
    );
    assert_eq!(
        Solution::rotate_string("abcde".to_string(), "abced".to_string()),
        false
    );
}
