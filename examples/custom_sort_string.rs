struct Solution {}
impl Solution {
    pub fn custom_sort_string(s: String, t: String) -> String {
        use std::collections::HashMap;

        let mut map = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            map.insert(c, i);
        }

        let mut t = t.chars().collect::<Vec<char>>();

        t.sort_by_key(|c| map.get(c).unwrap_or(&100));

        t.into_iter().collect()
    }
}

fn main() {
    assert_eq!(
        Solution::custom_sort_string("cba".to_string(), "abcd".to_string()),
        "cbad".to_string()
    );
}
