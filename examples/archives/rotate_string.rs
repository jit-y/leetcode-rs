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

    pub fn rotate_string_kmp(a: String, b: String) -> bool {
        let n = a.len();
        if n != b.len() {
            return false;
        }
        if n == 0 {
            return true;
        }

        let b = b.chars().collect::<Vec<char>>();
        let mut shifts = vec![1; n + 1];
        let mut left: i32 = -1;

        for right in 0..n {
            while left >= 0 && b[left as usize] != b[right] {
                left -= shifts[left as usize];
            }

            shifts[right + 1] = right as i32 - left;
            left += 1;
        }

        let mut match_len: i32 = 0;
        for c in format!("{}{}", a, a).chars() {
            while match_len >= 0 && b[match_len as usize] != c {
                match_len -= shifts[match_len as usize];
            }

            match_len += 1;
            if match_len == n as i32 {
                return true;
            }
        }

        false
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
    assert_eq!(
        Solution::rotate_string_kmp("abcde".to_string(), "cdeab".to_string()),
        true
    );
    assert_eq!(
        Solution::rotate_string_kmp("abcde".to_string(), "abced".to_string()),
        false
    );
}
