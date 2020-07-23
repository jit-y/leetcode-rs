struct Solution {}
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        use std::collections::HashMap;
        let mut dic = HashMap::new();

        for c in license_plate.chars() {
            if !c.is_alphabetic() {
                continue;
            }

            let v = c.to_ascii_lowercase();
            dic.entry(v).and_modify(|a| *a += 1).or_insert(1);
        }

        let mut words = words;

        words.sort_by_key(|w| w.len());

        for w in words {
            let mut map = HashMap::new();
            for c in w.chars() {
                map.entry(c).and_modify(|a| *a += 1).or_insert(1);
            }

            if dic.iter().all(|(c, n)| match map.get(&c) {
                None => false,
                Some(num) => num >= n,
            }) {
                return w;
            }
        }

        unreachable!()
    }
}

fn main() {
    assert_eq!(
        Solution::shortest_completing_word(
            "1s3 PSt".to_string(),
            vec!["step", "steps", "stripe", "stepple"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        "steps".to_string()
    );
    assert_eq!(
        Solution::shortest_completing_word(
            "1s3 456".to_string(),
            vec!["looks", "pest", "stew", "show"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        "pest".to_string()
    );
}
