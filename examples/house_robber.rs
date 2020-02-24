struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = Vec::new();

        match nums.first() {
            None => return 0,
            Some(x) => dp.push(*x),
        }

        match nums.get(1) {
            None => return dp[0],
            Some(x) => dp.push(std::cmp::max(dp[0], *x)),
        }

        for (i, n) in nums.into_iter().enumerate().skip(2) {
            dp.push(std::cmp::max(n + dp[i - 2], dp[i - 1]));
        }

        *dp.last().expect("something wrong")
    }
}

fn main() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}
