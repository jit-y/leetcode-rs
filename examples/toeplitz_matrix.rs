struct Solution {}
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        if matrix.is_empty() {
            return true;
        }

        matrix.iter().enumerate().skip(1).all(|(i, row)| {
            row.iter()
                .enumerate()
                .skip(1)
                .all(|(j, el)| &matrix[i - 1][j - 1] == el)
        })
    }
}

fn main() {
    assert_eq!(
        Solution::is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]]),
        true
    );
    assert_eq!(
        Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]),
        false
    );
}
