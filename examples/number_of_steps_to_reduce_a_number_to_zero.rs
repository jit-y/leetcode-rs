struct Solution {}
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut result = 0;

        while num > 0 {
            if num % 2 == 1 {
                num -= 1;
            } else {
                num /= 2;
            }

            result += 1;
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::number_of_steps(14), 6)
}
