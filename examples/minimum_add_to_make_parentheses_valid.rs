impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut result = 0;
        let mut balance = 0;

        for c in s.chars() {
            balance += if c == '(' { 1 } else { -1 };

            if balance == -1 {
                result += 1;
                balance += 1;
            }
        }

        result + balance
    }
}

fn main() {}
