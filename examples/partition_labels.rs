struct Solution {}
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            map.insert(c, i);
        }

        let mut result = vec![];
        let mut left = 0;
        let mut right = 0;

        for (i, c) in s.chars().enumerate() {
            let idx = *map.get(&c).expect("get");
            right = std::cmp::max(right, idx);

            if i == right {
                result.push((right - left + 1) as i32);

                left = right + 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
        vec![9, 7, 8]
    );
}
