struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut len = nums.len();
        let mut i = 0;

        while i < len {
            let x = nums[i];
            if x != 0 {
                i += 1;
                continue;
            }

            let r = nums.remove(i);
            nums.push(r);
            len -= 1;
        }
    }
}

fn main() {
    let mut a = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut a);
    assert_eq!(a, vec![1, 3, 12, 0, 0]);

    let mut b = vec![1, 0, 3, 12, 0];
    Solution::move_zeroes(&mut b);
    assert_eq!(b, vec![1, 3, 12, 0, 0]);
}
