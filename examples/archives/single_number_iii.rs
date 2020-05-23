struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut diff = nums.iter().fold(0, |acc, v| acc ^ v);
        diff &= -diff;
        let mut result = vec![0, 0];

        for n in nums {
            if diff & n != 0 {
                result[0] ^= n;
            } else {
                result[1] ^= n;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
}
