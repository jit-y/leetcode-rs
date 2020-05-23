struct Solution {}
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64;
        let mut left = 0;
        let mut right = (c as f64).sqrt() as i64;

        while left <= right {
            let v = left * left + right * right;
            if v == c {
                return true;
            }

            if v < c {
                left += 1;
            } else {
                right -= 1;
            }
        }

        false
    }

    pub fn judge_square_sum_bad2(c: i32) -> bool {
        let mut n = 0;
        while n * n <= c {
            let v = (c - n * n) as f32;
            let f = v.sqrt();

            if f == f.floor() {
                return true;
            }

            n += 1;
        }

        false
    }
    // bad solution
    pub fn judge_square_sum_bad(c: i32) -> bool {
        fn bs(left: i64, right: i64, target: i64) -> bool {
            if left > right {
                return false;
            }

            let mid = left + (right - left) / 2;
            if mid * mid == target as i64 {
                return true;
            }

            if mid * mid < target as i64 {
                bs(mid + 1, right, target)
            } else {
                bs(left, mid - 1, target)
            }
        }

        let mut n = 0;
        while n * n <= c {
            let v = (c - n * n) as i64;
            if bs(0, v, v) {
                return true;
            }

            n += 1;
        }

        false
    }
}

fn main() {
    assert_eq!(Solution::judge_square_sum(5), true);
    assert_eq!(Solution::judge_square_sum(3), false);
    assert_eq!(Solution::judge_square_sum(1), true);
    assert_eq!(Solution::judge_square_sum(2), true);
    assert_eq!(Solution::judge_square_sum(1000000000), true);
    assert_eq!(Solution::judge_square_sum(2147482647), false);
}
