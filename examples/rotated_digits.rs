struct Solution {}
impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut result = 0;

        for v in 1..=n {
            let mut a = v;
            let mut b = 0;
            let mut valid = true;
            let mut digit = (v as f64).log10() as i32 + 1;

            while a > 0 {
                let current = 10i64.pow(digit as u32) as i32;
                let c = match a / current {
                    0 => 0,
                    1 => 1,
                    2 => 5,
                    5 => 2,
                    6 => 9,
                    8 => 8,
                    9 => 6,
                    _ => {
                        valid = false;
                        break;
                    }
                };

                b += c * current;
                a %= current;
                digit -= 1;
            }

            if valid && v != b {
                result += 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::rotated_digits(10), 4);
}
