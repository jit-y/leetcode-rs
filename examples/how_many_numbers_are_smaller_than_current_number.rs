struct Solution {}
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = nums.clone();
        sorted.sort();

        let mut map = std::collections::HashMap::new();
        for i in 1..sorted.len() {
            if sorted[i] == sorted[i - 1] {
                continue;
            }

            map.insert(sorted[i], i as i32);
        }

        let mut result = vec![];

        for n in nums {
            match map.get(&n) {
                None => result.push(0),
                Some(v) => result.push(*v),
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
        vec![2, 1, 0, 3]
    );
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
        vec![0, 0, 0, 0]
    );
}
