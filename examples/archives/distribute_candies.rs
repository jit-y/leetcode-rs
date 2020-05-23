struct Solution {}
impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let mut uniq = std::collections::HashMap::new();

        for v in candies.iter() {
            uniq.insert(v, true);
        }

        std::cmp::min(candies.len() / 2, uniq.len()) as i32
    }
}

fn main() {
    assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
    assert_eq!(
        Solution::distribute_candies(vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3]),
        3
    )
}
