struct Solution {}
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut result = vec![0, 1];

        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                result[0] = nums[i];
            } else if nums[i] > nums[i - 1] + 1 {
                result[1] = nums[i - 1] + 1;
            }
        }

        if nums[nums.len() - 1] != nums.len() as i32 {
            result[1] = nums.len() as i32;
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), [2, 3]);
    assert_eq!(Solution::find_error_nums(vec![2, 2]), [2, 1]);
    assert_eq!(Solution::find_error_nums(vec![3, 2, 3, 4, 6, 5]), [3, 1]);
    assert_eq!(Solution::find_error_nums(vec![1, 1]), [1, 2]);
    assert_eq!(
        Solution::find_error_nums(vec![
            37, 62, 43, 27, 12, 66, 36, 18, 39, 54, 61, 65, 47, 32, 23, 2, 46, 8, 4, 24, 29, 38,
            63, 39, 25, 11, 45, 28, 44, 52, 15, 30, 21, 7, 57, 49, 1, 59, 58, 14, 9, 40, 3, 42, 56,
            31, 20, 41, 22, 50, 13, 33, 6, 10, 16, 64, 53, 51, 19, 17, 48, 26, 34, 60, 35, 5
        ]),
        [39, 55]
    );
}
