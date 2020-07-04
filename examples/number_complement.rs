struct Solution {}
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut n = num;
        let mut mask = 1;

        while n > 0 {
            n = n >> 1;
            mask = mask << 1;
        }

        num ^ mask - 1
    }
}

fn main() {
    assert_eq!(Solution::find_complement(5), 2);
}
