struct Solution {}
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }

        let n = n as i64;
        let mut left = 0 as i64;
        let mut right = n as i64;

        while left <= right {
            let mid = left + (right - left) / 2;
            let coin_n = mid * (mid + 1) / 2;

            if coin_n > n {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        right as i32
    }
}

fn main() {
    assert_eq!(Solution::arrange_coins(5), 2);
    assert_eq!(Solution::arrange_coins(8), 3);
    assert_eq!(Solution::arrange_coins(0), 0);
    assert_eq!(Solution::arrange_coins(846930886), 41156);
}
