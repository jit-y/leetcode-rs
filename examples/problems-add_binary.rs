struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        fn _add_binary(a: &mut Vec<char>, b: &mut Vec<char>, result: &mut String) {
            let mut carry = 0;
            for dc in a.into_iter().rev() {
                let mut d1 = dc.to_digit(2).expect("not digit");
                d1 = d1 + carry;
                carry = 0;

                if d1 == 2 {
                    d1 = 0;
                    carry = 1;
                }

                match b.pop() {
                    Some(d2) => {
                        let d2 = d2.to_digit(2).expect("not digit");
                        if d1 + d2 == 2 {
                            carry = 1;
                            result.insert(0, '0');
                        } else {
                            let c = std::char::from_digit(d1 + d2, 2).expect("not char");
                            result.insert(0, c);
                        }
                    }
                    None => {
                        let dc = std::char::from_digit(d1, 2).expect("not char");
                        result.insert(0, dc);
                    }
                }
            }

            if carry == 1 {
                result.insert(0, '1');
            }
        }

        let mut result = String::new();
        let mut a = a.chars().collect::<Vec<char>>();
        let a_size = a.len();
        let mut b = b.chars().collect::<Vec<char>>();
        let b_size = b.len();

        if a_size >= b_size {
            _add_binary(&mut a, &mut b, &mut result);
        } else {
            _add_binary(&mut b, &mut a, &mut result);
        }

        result
    }

    // thread 'main' panicked at 'attempt to multiply with overflow'
    pub fn add_binary_b(a: String, b: String) -> String {
        let a = a.chars();
        let b = b.chars();

        let mut a_result: u64 = 0;
        for (i, c) in a.rev().enumerate() {
            a_result += c.to_digit(2).expect("something wrong") as u64 * 2_u64.pow(i as u32);
        }

        let mut b_result: u64 = 0;
        for (i, c) in b.rev().enumerate() {
            b_result += c.to_digit(2).expect("something wrong") as u64 * 2_u64.pow(i as u32);
        }

        format!("{:b}", a_result + b_result)
    }
}

fn main() {
    println!(
        "{}",
        Solution::add_binary("1010".to_string(), "1011".to_string())
    );
    println!(
        "{}",
        Solution::add_binary("11".to_string(), "1".to_string())
    );
    println!(
        "{}",
        Solution::add_binary(
            "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(),
            "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string()
        )
    );
    println!("{}", Solution::add_binary("0".to_string(), "0".to_string()));
}
