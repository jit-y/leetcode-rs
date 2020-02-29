struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for n in nums {
            // 2 times
            b |= a & n;

            // 1 time
            a ^= n;

            // 3 times
            let c = a & b;

            a &= !c;
            b &= !c;
        }

        a
    }
}

fn main() {
    assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    assert_eq!(Solution::single_number(vec![2, 3, 2, 3, 2, 3, 99]), 99);
}
