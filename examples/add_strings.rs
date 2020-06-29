struct Solution {}
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut num1 = num1.chars().collect::<Vec<char>>();
        let mut num2 = num2.chars().collect::<Vec<char>>();

        num1.reverse();
        num2.reverse();

        let max_len = std::cmp::max(num1.len(), num2.len());
        let mut carry = false;
        let mut result = String::new();

        for i in 0..max_len {
            let n1 = match num1.get(i) {
                Some(v) => v.to_digit(10).unwrap(),
                None => 0,
            };
            let n2 = match num2.get(i) {
                Some(v) => v.to_digit(10).unwrap(),
                None => 0,
            };

            let mut n = n1 + n2;
            if carry {
                n += 1;
                carry = false;
            }
            if n > 9 {
                carry = true;
                n = n % 10;
            }

            result.insert(0, std::char::from_digit(n, 10).unwrap());
        }

        if carry {
            result.insert(0, '1');
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::add_strings("0".to_string(), "0".to_string()),
        "0".to_string()
    );
    assert_eq!(
        Solution::add_strings("9".to_string(), "1".to_string()),
        "10".to_string()
    );
    assert_eq!(
        Solution::add_strings("10204050".to_string(), "3049596".to_string()),
        "13253646".to_string()
    );
}
