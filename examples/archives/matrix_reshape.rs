struct Solution {}
impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return nums;
        }

        let total = nums.len() * nums[0].len();
        if r * c != total as i32 {
            return nums;
        }

        let mut result = Vec::new();
        let mut lim = c;
        let mut arr = Vec::new();
        for row in nums {
            for el in row {
                arr.push(el);
                lim -= 1;

                if lim == 0 {
                    result.push(arr);
                    arr = Vec::new();
                    lim = c;
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
        vec![vec![1, 2, 3, 4]]
    );
    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]], 2, 4),
        vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]]
    );
    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
        vec![vec![1, 2], vec![3, 4]]
    );
}
