struct Solution {}
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut result = vec![];

        let first = first.as_str();
        let second = second.as_str();
        let words = text.split_ascii_whitespace().collect::<Vec<&str>>();

        for i in 2..words.len() {
            if words[i - 2] != first {
                continue;
            }
            if words[i - 1] != second {
                continue;
            }

            result.push(words[i].to_string());
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::find_ocurrences(
            "alice is a good girl she is a good student".to_string(),
            "a".to_string(),
            "good".to_string()
        ),
        ["girl".to_string(), "student".to_string()]
    );
    assert_eq!(
        Solution::find_ocurrences(
            "we will we will rock you".to_string(),
            "we".to_string(),
            "will".to_string()
        ),
        ["we".to_string(), "rock".to_string()]
    );
    assert_eq!(
        Solution::find_ocurrences(
            "jkypmsxd jkypmsxd kcyxdfnoa jkypmsxd kcyxdfnoa jkypmsxd kcyxdfnoa kcyxdfnoa jkypmsxd kcyxdfnoa".to_string(),
            "kcyxdfnoa".to_string(),
            "jkypmsxd".to_string()
        ),
        ["kcyxdfnoa".to_string(),"kcyxdfnoa".to_string(),"kcyxdfnoa".to_string()]
    );
}
