struct Solution {}
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut v = x ^ y;
        let mut distance = 0;

        while v > 0 {
            if v & 1 == 1 {
                distance += 1;
            }
            v = v >> 1;
        }

        distance
    }
}

fn main() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
}
