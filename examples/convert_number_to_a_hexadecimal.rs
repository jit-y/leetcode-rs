struct Solution {}
impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut num = num as i64;
        let mut result = String::new();

        if num < 0 {
            num = 0xffffffff + num + 1;
        }

        while num > 0 {
            let v = num % 16;
            result.insert(0, std::char::from_digit(v as u32, 16).expect("from_digit"));
            num = num >> 4;
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::to_hex(26), "1a".to_string());
    assert_eq!(Solution::to_hex(2345), "929".to_string());
    assert_eq!(Solution::to_hex(-1), "ffffffff".to_string());
}
