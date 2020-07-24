struct Solution {}
impl Solution {
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let mut result = 0;
        let candidates = vec![2, 3, 5, 7, 11, 13, 17, 19];

        for mut n in l..=r {
            let mut b = 0;

            while n > 0 {
                b += n & 1;
                n = n >> 1;
            }

            if candidates.contains(&b) {
                result += 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
    assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
}
