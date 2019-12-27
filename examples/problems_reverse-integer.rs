struct Solution {}
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut result = 0;

        loop {
            if num == 0 {
                break;
            }

            let pop_val = num % 10;
            num /= 10;

            // 2 ** 31 - 1 == 2147483647
            if result > i32::max_value() / 10 || (result == i32::max_value() && pop_val > 7) {
                return 0;
            }

            // -(2 ** 31) == -2147483648
            if result < i32::min_value() / 10 || (result == i32::min_value() && pop_val < -8) {
                return 0;
            }

            result = result * 10 + pop_val;
        }

        result
    }
}
