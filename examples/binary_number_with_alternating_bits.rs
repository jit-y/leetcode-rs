struct Solution {}
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut x = n & 1;

        while n > 1 {
            n = n >> 1;
            let y = n & 1;

            if x ^ y != 1 {
                return false;
            }

            x = y;
        }

        true
    }
}

fn main() {
    assert_eq!(Solution::has_alternating_bits(5), true);
    assert_eq!(Solution::has_alternating_bits(7), false);
    assert_eq!(Solution::has_alternating_bits(11), false);
    assert_eq!(Solution::has_alternating_bits(42), true);
    assert_eq!(Solution::has_alternating_bits(1), true);
    assert_eq!(Solution::has_alternating_bits(0), true);
    assert_eq!(Solution::has_alternating_bits(8), false);
}
