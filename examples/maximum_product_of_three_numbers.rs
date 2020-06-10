struct Solution {}
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();

        nums.sort();

        std::cmp::max(
            nums[0] * nums[1] * nums[len - 1],
            nums[len - 1] * nums[len - 2] * nums[len - 3],
        )
    }

    pub fn maximum_product_2(nums: Vec<i32>) -> i32 {
        let mut min_1 = 1000;
        let mut min_2 = 1000;
        let mut max_1 = -1000;
        let mut max_2 = -1000;
        let mut max_3 = -1000;

        for n in nums {
            if n <= min_1 {
                min_2 = min_1;
                min_1 = n;
            } else if n <= min_2 {
                min_2 = n;
            }

            if n >= max_1 {
                max_3 = max_2;
                max_2 = max_1;
                max_1 = n;
            } else if n >= max_2 {
                max_3 = max_2;
                max_2 = n;
            } else if n >= max_3 {
                max_3 = n;
            }
        }

        std::cmp::max(min_1 * min_2 * max_1, max_1 * max_2 * max_3)
    }
}

fn main() {
    assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
    assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
    assert_eq!(Solution::maximum_product(vec![-4, -3, -2, -1, 60]), 720);
    assert_eq!(Solution::maximum_product_2(vec![1, 2, 3]), 6);
    assert_eq!(Solution::maximum_product_2(vec![1, 2, 3, 4]), 24);
    assert_eq!(Solution::maximum_product_2(vec![-4, -3, -2, -1, 60]), 720);
}
