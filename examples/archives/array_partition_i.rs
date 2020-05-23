struct Solution {}
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort();

        nums.into_iter().step_by(2).fold(0, |acc, n| acc + n)
    }
}

fn main() {
    assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
}
