struct Solution {}
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        let mut result = Vec::new();

        for n in nums1.into_iter() {
            map.insert(n, true);
        }

        for n in nums2.into_iter() {
            if let Some(_) = map.remove(&n) {
                result.push(n);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2]
    );
}
