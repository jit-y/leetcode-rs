struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

fn main() {
    assert_eq!(
        Solution::reverse_words("the sky is blue".to_string()),
        "blue is sky the".to_string()
    );
    assert_eq!(
        Solution::reverse_words("  hello world!  ".to_string()),
        "world! hello".to_string()
    );
    assert_eq!(
        Solution::reverse_words("a good   example".to_string()),
        "example good a".to_string()
    );
}
