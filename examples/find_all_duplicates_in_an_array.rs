struct Solution {}
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut result = Vec::new();

        for i in 0..nums.len() {
            let v = (nums[i].abs() - 1) as usize;
            if nums[v] > 0 {
                nums[v] = -nums[v];
                continue;
            }

            result.push(v as i32 + 1);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![2, 3]
    );
    assert_eq!(
        Solution::find_duplicates(vec![10, 2, 5, 10, 9, 1, 1, 4, 3, 7]),
        vec![10, 1]
    );
}
