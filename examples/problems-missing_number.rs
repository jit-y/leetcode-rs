struct Solution {}
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let ideal: i32 = (1..=nums.len()).sum::<usize>() as i32;
        let actual: i32 = nums.iter().sum();

        ideal - actual
    }
}

fn main() {
    assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
}
