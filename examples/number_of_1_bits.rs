struct Solution {}
impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut n = n;
        let mut result = 0;

        while n > 0 {
            result += n & 1;
            n = n >> 1;
        }

        result as i32
    }
}

fn main() {
    assert_eq!(
        Solution::hammingWeight(0b00000000000000000000000000001011),
        3
    );
    assert_eq!(
        Solution::hammingWeight(0b00000000000000000000000010000000),
        1
    );
    assert_eq!(
        Solution::hammingWeight(0b11111111111111111111111111111101),
        31
    );
}
