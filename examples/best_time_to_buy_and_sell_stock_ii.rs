struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                result += prices[i] - prices[i - 1];
            }
        }

        result
    }

    pub fn max_profit_2(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut i = 0;

        while i < prices.len() - 1 {
            while i < prices.len() - 1 && prices[i] >= prices[i + 1] {
                i += 1;
            }
            let valley = prices[i];

            while i < prices.len() - 1 && prices[i] <= prices[i + 1] {
                i += 1;
            }
            let peek = prices[i];

            result += peek - valley;
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(Solution::max_profit_2(vec![7, 1, 5, 3, 6, 4]), 7);
}
