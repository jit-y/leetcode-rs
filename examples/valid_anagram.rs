struct Solution {}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = std::collections::HashMap::new();

        for sc in s.chars() {
            *map.entry(sc).or_insert(0) += 1;
        }

        for tc in t.chars() {
            match map.get_mut(&tc) {
                None => return false,
                Some(el) => {
                    *el -= 1;
                    if *el == 0 {
                        map.remove(&tc);
                    }
                }
            }
        }

        map.is_empty()
    }
}

fn main() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    );
    assert_eq!(
        Solution::is_anagram("aacc".to_string(), "ccac".to_string()),
        false
    );
}
