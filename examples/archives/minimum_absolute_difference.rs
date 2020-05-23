struct Solution {}
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort();
        let mut min = 1000000;
        for i in 1..arr.len() {
            min = std::cmp::min(min, (arr[i - 1] - arr[i]).abs());
        }

        let mut result = vec![];

        for i in 1..arr.len() {
            if (arr[i - 1] - arr[i]).abs() == min {
                result.push(vec![arr[i - 1], arr[i]]);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
        vec![vec![1, 2], vec![2, 3], vec![3, 4]]
    );
    assert_eq!(
        Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]),
        vec![vec![1, 3]]
    );
    assert_eq!(
        Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
        vec![vec![-14, -10], vec![19, 23], vec![23, 27]]
    );
}
