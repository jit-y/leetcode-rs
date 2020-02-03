struct Solution {}
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num < 10 {
            return num;
        }
        let mut current = 0;
        let mut num = num;

        while num >= 10 {
            while num > 0 {
                current += num % 10;
                num /= 10;
            }

            if current < 10 {
                break;
            }

            num = current;
            current = 0;
        }

        current
    }

    // dr(n): 0 if n == 0
    // dr(n): 1 + ((n - 1) mod 9) if n != 0
    pub fn add_digits_digital_root(num: i32) -> i32 {
        if num <= 0 {
            return num;
        }

        1 + (num - 1) % 9
    }
}

fn main() {
    assert_eq!(Solution::add_digits(38), 2);
    assert_eq!(Solution::add_digits_digital_root(38), 2);
    assert_eq!(Solution::add_digits(1), 1);
    assert_eq!(Solution::add_digits_digital_root(1), 1);
}
