struct Solution {}
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut result = -1;
        let mut map = std::collections::HashMap::new();

        for n in arr {
            *map.entry(n).or_insert(0) += 1;
        }

        for (k, v) in map {
            if k != v {
                continue;
            }

            result = std::cmp::max(result, k);
        }

        result
    }

    pub fn find_lucky_2(arr: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();

        for n in arr {
            *map.entry(n).or_insert(0) += 1;
        }

        map.into_iter()
            .filter(|(k, v)| k == v)
            .map(|(k, _)| k)
            .max()
            .unwrap_or(-1)
    }
}

fn main() {
    assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
    assert_eq!(Solution::find_lucky_2(vec![2, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lucky_2(vec![1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(Solution::find_lucky_2(vec![2, 2, 2, 3, 3]), -1);
}
