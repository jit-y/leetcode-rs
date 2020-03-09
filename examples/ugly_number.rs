// https://leetcode.com/problems/ugly-number
// 2 ^ x * 3 ^ y * 5 ^ z
struct Solution {}
impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num < 1 {
            return false;
        }

        let mut num = num;
        let candidates = vec![2, 3, 5];
        for i in candidates {
            while num % i == 0 {
                num /= i;
            }
        }

        num == 1
    }
}

fn main() {
    assert_eq!(Solution::is_ugly(1), true);
    assert_eq!(Solution::is_ugly(4), true);
    assert_eq!(Solution::is_ugly(6), true);
    assert_eq!(Solution::is_ugly(14), false);
    assert_eq!(Solution::is_ugly(15), true);
}
