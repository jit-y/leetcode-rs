struct Solution {}
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }

    pub fn single_non_duplicate_2(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mut mid = (left + right) / 2;
            if mid % 2 == 1 {
                mid -= 1;
            }

            if nums[mid] != nums[mid + 1] {
                right = mid;
            } else {
                left = mid + 2;
            }
        }

        nums[left]
    }
}
// 112233445

fn main() {
    assert_eq!(
        Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
        2
    );
    assert_eq!(
        Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
        10
    );
    assert_eq!(
        Solution::single_non_duplicate_2(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
        2
    );
    assert_eq!(
        Solution::single_non_duplicate_2(vec![3, 3, 7, 7, 10, 11, 11]),
        10
    );
}
