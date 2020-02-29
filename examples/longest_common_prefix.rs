// https://leetcode.com/problems/longest-common-prefix/
struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::new();
        if strs.is_empty() {
            return result;
        }

        let mut map = std::collections::HashMap::new();
        let x = &strs[0];

        for (i, c) in x.chars().enumerate() {
            map.insert(i, c);
        }

        for s in strs.iter().skip(1) {
            let chars = s.chars().collect::<Vec<char>>();

            for i in 0..map.len() {
                match chars.get(i) {
                    None => {
                        map.remove(&i);
                    }
                    Some(v) => {
                        if let Some(c) = map.get(&i) {
                            if v != c {
                                map.remove(&i);
                            }
                        }
                    }
                }
            }
        }

        for (i, c) in x.chars().enumerate() {
            match map.get(&i) {
                None => break,
                Some(n) => {
                    if *n != c {
                        break;
                    }
                    result.push(c);
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl".to_string()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        "".to_string()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec!["aa".to_string(), "aa".to_string()]),
        "aa".to_string()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec!["aa".to_string(), "a".to_string()]),
        "a".to_string()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec!["ab".to_string(), "ac".to_string()]),
        "a".to_string()
    );
}
