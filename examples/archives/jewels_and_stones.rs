struct Solution {}
impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut map: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

        j.chars().fold(0, |acc, c| acc + *map.entry(c).or_default())
    }
}

fn main() {
    assert_eq!(
        Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
    assert_eq!(
        Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()),
        0
    );
}
