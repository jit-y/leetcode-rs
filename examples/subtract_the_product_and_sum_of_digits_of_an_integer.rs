struct Solution {}
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        let mut n = n;
        let mut prd = 1;
        let mut sum = 0;

        while n > 0 {
            let v = n % 10;
            prd *= v;
            sum += v;

            n /= 10;
        }

        prd - sum
    }
}

fn main() {
    assert_eq!(Solution::subtract_product_and_sum(234), 15);
}
