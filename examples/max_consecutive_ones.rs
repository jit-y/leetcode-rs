struct Solution {}
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut count = 0;

        for n in nums {
            match n {
                0 => count = 0,
                1 => count += 1,
                _ => unreachable!(),
            }

            result = std::cmp::max(result, count);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
        3
    );
}
