struct Solution {}
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max = 0;

        for (i, n) in arr.into_iter().enumerate() {
            max = std::cmp::max(max, n);

            if max == i as i32 {
                result += 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 3, 4, 2]), 2);
}
