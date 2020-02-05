struct Solution {}
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;

        while b != 0 {
            let carry = a & b;

            a ^= b;

            b = carry << 1;
        }

        a
    }
}

fn main() {
    assert_eq!(Solution::get_sum(1, 2), 3);
    assert_eq!(Solution::get_sum(100, 34), 134);
}
