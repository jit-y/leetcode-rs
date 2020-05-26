struct Solution {}
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut prev = ' ';
        let mut count = 1;
        let mut result = 0;

        for c in s.chars() {
            if c == prev {
                count += 1;
            } else {
                count = 1;
            }

            prev = c;
            result = std::cmp::max(result, count);
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::max_power("leetcode".to_string()), 2);
    assert_eq!(Solution::max_power("abbcccddddeeeeedcba".to_string()), 5);
    assert_eq!(Solution::max_power("triplepillooooow".to_string()), 5);
}
