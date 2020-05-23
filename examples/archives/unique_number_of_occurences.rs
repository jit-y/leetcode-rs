struct Solution {}
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::{HashMap, HashSet};
        let mut map = HashMap::new();

        for v in arr {
            *map.entry(v).or_insert(0) += 1;
        }

        let a = map.len();
        let o: HashSet<&i32> = map.values().collect();

        a == o.len()
    }
}

fn main() {
    assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
    assert_eq!(
        Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
        true
    );
}
