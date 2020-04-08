struct Solution {}
impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        if a.is_empty() {
            return vec![];
        }
        let mut words = std::collections::HashSet::new();
        let mut maps = vec![];
        for s in a {
            let mut m = std::collections::HashMap::new();
            for c in s.chars() {
                *m.entry(c).or_insert(0) += 1;
                words.insert(c);
            }

            maps.push(m);
        }

        let mut result = vec![];
        for c in words {
            let mut v = 100;
            for map in maps.iter() {
                match map.get(&c) {
                    None => {
                        v = 0;
                        break;
                    }
                    Some(x) => {
                        v = std::cmp::min(v, *x);
                    }
                }
            }

            for _ in 0..v {
                result.push(c.to_string());
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::common_chars(
            vec!["bella", "label", "roller"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        ),
        vec!["e", "l", "l"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::common_chars(
            vec!["cool", "lock", "cook"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        ),
        vec!["c", "o"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
}
