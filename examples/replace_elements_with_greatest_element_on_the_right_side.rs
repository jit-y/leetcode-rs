struct Solution {}
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut current_max = -1;

        for n in arr.into_iter().rev() {
            result.insert(0, current_max);

            current_max = std::cmp::max(current_max, n);
        }

        result
    }

    pub fn replace_elements_2(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut current_max = -1;

        for i in (0..arr.len()).rev() {
            let n = arr[i];

            arr[i] = current_max;

            current_max = std::cmp::max(current_max, n);
        }

        arr
    }
}

fn main() {
    assert_eq!(
        Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    );
    assert_eq!(
        Solution::replace_elements_2(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    );
}
