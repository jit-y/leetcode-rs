struct Solution {}
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut result = 1;
        let mut current = 1;
        for i in 1..nums.len() {
            let a = nums[i - 1];
            let b = nums[i];

            if a < b {
                current += 1;
            } else {
                current = 1;
            }

            result = std::cmp::max(result, current);
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
    assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
    assert_eq!(Solution::find_length_of_lcis(vec![]), 0);
    assert_eq!(Solution::find_length_of_lcis(vec![1]), 1);
    assert_eq!(Solution::find_length_of_lcis(vec![1, 3]), 2);
}
