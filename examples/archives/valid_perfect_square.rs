// https://leetcode.com/problems/valid-perfect-square/
struct Solution {}
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }

        let mut left = 1;
        let mut right = num;
        let mut result = false;

        while left <= right {
            let mid = left + (right - left) / 2;

            if mid * mid == num {
                result = true;
                break;
            }

            if mid < num / mid {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::is_perfect_square(16), true);
    assert_eq!(Solution::is_perfect_square(14), false);
    assert_eq!(Solution::is_perfect_square(5), false);
}
