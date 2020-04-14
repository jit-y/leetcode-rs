struct Solution {}
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut result = 0;
        let mut idx = -1;

        for i in 0..32 {
            if (n >> i & 1) > 0 {
                if idx >= 0 {
                    result = std::cmp::max(result, i - idx);
                }

                idx = i
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::binary_gap(22), 2);
    assert_eq!(Solution::binary_gap(5), 2);
    assert_eq!(Solution::binary_gap(6), 1);
    assert_eq!(Solution::binary_gap(8), 0);
}
