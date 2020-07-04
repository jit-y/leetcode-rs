struct Solution {}
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let min = nums[0];
        let mut result = 0;

        for n in nums {
            result += n - min;
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
}
