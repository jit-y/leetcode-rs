struct Solution {}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            if let Some(v) = nums.pop() {
                nums.insert(0, v);
            }
        }
    }

    pub fn rotate_rev(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len();
        let k = k as usize % l;

        nums[0..l].reverse();
        nums[0..k].reverse();
        nums[k..l].reverse();
    }
}

fn main() {
    let mut a = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut a, 3);
    assert_eq!(a, vec![5, 6, 7, 1, 2, 3, 4]);

    let mut a = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate_rev(&mut a, 3);
    assert_eq!(a, vec![5, 6, 7, 1, 2, 3, 4]);
}
