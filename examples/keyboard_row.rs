struct Solution {}
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut results = words;
        let mut map = std::collections::HashMap::new();
        for (i, s) in ["qwertyuiop", "asdfghjkl", "zxcvbnm"].iter().enumerate() {
            for c in s.chars() {
                map.insert(c, i);
            }
        }

        results.retain(|s| {
            let cs = s.chars().collect::<Vec<char>>();
            for i in 1..cs.len() {
                let a = map.get(&cs[i - 1].to_ascii_lowercase()).unwrap();
                let b = map.get(&cs[i].to_ascii_lowercase()).unwrap();
                if a != b {
                    return false;
                }
            }

            true
        });

        results
    }
}

fn main() {
    assert_eq!(
        Solution::find_words(vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string()
        ]),
        vec!["Alaska".to_string(), "Dad".to_string()]
    );
}
