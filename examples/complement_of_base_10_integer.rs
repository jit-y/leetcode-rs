struct Solution {}
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let mut mask = 1;
        let mut v = n;
        while v > 0 {
            mask *= 2;
            v = v >> 1;
        }

        mask -= 1;

        n ^ mask
    }
}

fn main() {
    assert_eq!(Solution::bitwise_complement(5), 2);
    assert_eq!(Solution::bitwise_complement(7), 0);
    assert_eq!(Solution::bitwise_complement(10), 5);
    assert_eq!(Solution::bitwise_complement(100), 27);
    assert_eq!(Solution::bitwise_complement(0), 1);
}
