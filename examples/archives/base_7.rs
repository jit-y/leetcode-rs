struct Solution {}
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        fn convert(num: i32) -> String {
            if num == 0 {
                return String::new();
            }

            format!("{}{}", convert(num / 7), num % 7)
        }

        if num == 0 {
            return "0".to_string();
        }

        if num < 0 {
            format!("-{}", convert(num.abs()))
        } else {
            convert(num)
        }
    }
}

fn main() {
    assert_eq!(Solution::convert_to_base7(100), "202".to_string());
    assert_eq!(Solution::convert_to_base7(-7), "-10".to_string());
    assert_eq!(Solution::convert_to_base7(0), "0".to_string());
}
