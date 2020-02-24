struct Solution {}
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        if s.is_empty() {
            return -1;
        }
        let mut arr = vec![None; 255];

        for c in s.chars() {
            *arr[c as usize].get_or_insert(0) += 1;
        }

        let mut result = -1;

        for (i, c) in s.chars().enumerate() {
            match arr[c as usize] {
                None => continue,
                Some(count) => {
                    if count == 1 {
                        result = i as i32;
                        break;
                    }
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(Solution::first_uniq_char("aadadaad".to_string()), -1);
}
