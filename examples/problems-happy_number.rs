struct Solution {}
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut result = n;
        let mut memo = std::collections::HashMap::new();

        while result > 0 {
            let mut current = result;
            result = 0;

            while current > 0 {
                result += (current % 10).pow(2);
                current /= 10;
            }

            if let Some(_) = memo.get(&result) {
                return false;
            }

            if result == 1 {
                return true;
            } else {
                memo.insert(result, true);
            }
        }

        false
    }
}

fn main() {
    println!("{}", Solution::is_happy(19));
    println!("{}", Solution::is_happy(1));
    println!("{}", Solution::is_happy(2));
    println!("{}", Solution::is_happy(1234));
}
