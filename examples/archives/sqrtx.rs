// https://leetcode.com/problems/sqrtx/
struct Solution {}
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 {
            return x;
        }

        let mut left = 1;
        let mut right = x;
        let mut result = right;

        while left <= right {
            let mid = left + (right - left) / 2;

            // mid * mid will cause overflow.
            if mid <= x / mid {
                left = mid + 1;
                result = mid;
            } else {
                right = mid - 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(3), 1);
    assert_eq!(Solution::my_sqrt(8), 2);
    assert_eq!(Solution::my_sqrt(16), 4);
    assert_eq!(Solution::my_sqrt(2147395599), 46339);
}
