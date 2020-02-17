struct Solution {}
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = Vec::new();

        match cost.first() {
            None => return 0,
            Some(v) => dp.push(*v),
        }

        match cost.get(1) {
            None => return dp[0],
            Some(n) => dp.push(*n + std::cmp::min(0, dp[0])),
        }

        for (i, n) in cost.into_iter().enumerate().skip(2) {
            let v = n + std::cmp::min(dp[i - 2], dp[i - 1]);
            dp.push(v);
        }

        let len = dp.len();
        std::cmp::min(dp[len - 1], dp[len - 2])
    }

    pub fn min_cost_climbing_stairs_2(cost: Vec<i32>) -> i32 {
        let mut cost = cost;

        for i in 2..cost.len() {
            cost[i] += std::cmp::min(cost[i - 1], cost[i - 2]);
        }

        std::cmp::min(cost[cost.len() - 1], cost[cost.len() - 2])
    }
}

fn main() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}
