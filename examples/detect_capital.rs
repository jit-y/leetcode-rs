struct Solution {}
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut chars = word.chars();
        let a = match chars.next() {
            None => return true,
            Some(v) => v,
        };
        let b = match chars.next() {
            None => return true,
            Some(v) => v,
        };

        match (a.is_ascii_uppercase(), b.is_ascii_uppercase()) {
            (true, true) => chars.all(|c| c.is_ascii_uppercase()),
            (_, false) => chars.all(|c| c.is_ascii_lowercase()),
            _ => false,
        }
    }
}

fn main() {
    assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
    assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
}
