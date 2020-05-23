struct Solution {}
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        let mut result = Vec::new();

        for n in nums1.into_iter() {
            *map.entry(n).or_insert(0) += 1;
        }

        for n in nums2.into_iter() {
            if let Some(v) = map.get_mut(&n) {
                if *v >= 1 {
                    result.push(n);
                    *v -= 1;
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2, 2]
    );
    assert_eq!(
        Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
        vec![9, 4]
    );
}
