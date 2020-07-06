struct Solution {}
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        use std::collections::HashMap;
        let mut map: HashMap<char, i32> = HashMap::new();
        let s = s.chars().collect::<Vec<char>>();

        for i in 0..s.len() {
            map.entry(s[i]).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut map: Vec<_> = map.into_iter().collect();
        map.sort_by(|a, b| (-a.1).cmp(&-b.1));

        let mut result = String::new();

        for (c, n) in map {
            for _ in 0..n {
                result.push(c);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::frequency_sort("tree".to_string()),
        "eert".to_string()
    );
    assert_eq!(
        Solution::frequency_sort("Aabb".to_string()),
        "bbAa".to_string()
    );
}
