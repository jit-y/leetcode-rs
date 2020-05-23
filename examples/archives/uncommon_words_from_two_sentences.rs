struct Solution {}
impl Solution {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let mut map = std::collections::HashMap::new();

        for s in a.split_ascii_whitespace().into_iter() {
            *map.entry(s).or_insert(0) += 1;
        }

        for s in b.split_ascii_whitespace().into_iter() {
            *map.entry(s).or_insert(0) += 1;
        }

        let mut result = vec![];
        for (k, v) in map {
            if v == 1 {
                result.push(k.to_string());
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string(),
        ),
        ["sweet".to_string(), "sour".to_string()]
    );
    assert_eq!(
        Solution::uncommon_from_sentences("apple apple".to_string(), "bannana".to_string(),),
        ["bannana".to_string()]
    );
}
