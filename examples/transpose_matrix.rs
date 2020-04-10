struct Solution {}
impl Solution {
    pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if a.is_empty() {
            return result;
        }

        let row_len = a.len();
        let col_len = a[0].len();

        for c in 0..col_len {
            let mut arr = vec![];

            for r in 0..row_len {
                arr.push(a[r][c]);
            }

            result.push(arr);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    );
    assert_eq!(
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
}
