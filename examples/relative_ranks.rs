struct Solution {}
impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut sorted = nums.clone();
        sorted.sort();
        let mut map = std::collections::HashMap::new();
        for (i, v) in sorted.into_iter().rev().enumerate() {
            map.insert(v, i);
        }

        for v in nums {
            let i = map.get(&v).expect("get");
            let val = match *i {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                x => (x + 1).to_string(),
            };

            result.push(val);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
        vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
        vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
}
