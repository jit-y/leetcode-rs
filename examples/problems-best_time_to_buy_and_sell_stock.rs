struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut buy = match prices.first() {
            None => return 0,
            Some(x) => *x,
        };

        for sell in prices.into_iter().skip(1) {
            let profit = sell - buy;
            if profit < 0 {
                buy = sell;
                continue;
            }

            if profit > result {
                result = profit;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 5, 6, 1, 3, 6, 4]), 5);
}
