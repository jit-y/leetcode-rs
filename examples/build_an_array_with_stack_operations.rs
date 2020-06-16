struct Solution {}
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut current = 1;
        let mut result = vec![];

        for v in target {
            for c in current..=n {
                result.push("Push".to_string());

                if c == v {
                    current = c + 1;
                    break;
                }

                result.push("Pop".to_string());
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::build_array(vec![1, 3], 3),
        vec!["Push", "Push", "Pop", "Push"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::build_array(vec![1, 2, 3], 3),
        vec!["Push", "Push", "Push"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::build_array(vec![1, 2], 4),
        vec!["Push", "Push"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::build_array(vec![2, 3, 4], 4),
        vec!["Push", "Pop", "Push", "Push", "Push"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
}
