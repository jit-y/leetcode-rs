struct Solution {}
impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut n = num;
        let mut digit_num = 1;
        let mut target = 0;

        while n > 0 {
            let c = n % 10;

            if c == 6 {
                target = digit_num;
            }

            digit_num *= 10;
            n /= 10;
        }

        let mut result = num;
        result += target * 3;

        result
    }
}

fn main() {
    assert_eq!(Solution::maximum69_number(9669), 9969);
    assert_eq!(Solution::maximum69_number(9996), 9999);
    assert_eq!(Solution::maximum69_number(9999), 9999);
}
