struct Solution {}
impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let s = str.split_whitespace().collect::<Vec<&str>>();
        if s.len() != pattern.len() {
            return false;
        }
        let p = pattern.chars().collect::<Vec<char>>();

        let mut s_m = std::collections::HashMap::new();
        let mut p_m = std::collections::HashMap::new();

        for i in 0..s.len() {
            s_m.insert(s[i], p[i]);
            p_m.insert(p[i], s[i]);
        }

        for i in 0..s.len() {
            if s_m.get(&s[i]) != p.get(i) || p_m.get(&p[i]) != s.get(i) {
                return false;
            }
        }

        true
    }
}

fn main() {
    assert_eq!(
        Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
        true
    );
    assert_eq!(
        Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
        false
    );
    assert_eq!(
        Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
        false
    );
    assert_eq!(
        Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
        false
    );
}
