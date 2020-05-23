struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = String::new();
        let mut s = s.split_ascii_whitespace();
        match s.next() {
            None => {
                return result;
            }
            Some(w) => {
                for c in w.chars().rev() {
                    result.push(c);
                }
            }
        }

        loop {
            match s.next() {
                None => break,
                Some(w) => {
                    result.push(' ');

                    for c in w.chars().rev() {
                        result.push(c);
                    }
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::reverse_words("Let's take LeetCode contest".to_string()),
        "s'teL ekat edoCteeL tsetnoc".to_string()
    );
}
