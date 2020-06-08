struct Solution {}
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        for i in 0..nums.len() {
            let n = nums[i];
            let idx = index[i] as usize;

            result.insert(idx, n);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
        vec![0, 4, 1, 3, 2]
    );
}
