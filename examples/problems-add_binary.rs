struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
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
}
