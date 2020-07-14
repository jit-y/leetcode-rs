struct Solution {}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            current: &mut Vec<i32>,
            i: usize,
            sum: i32,
            candidates: &Vec<i32>,
            target: i32,
            result: &mut Vec<Vec<i32>>,
        ) {
            if i >= candidates.len() {
                return;
            }

            if sum > target {
                return;
            }

            if sum == target {
                result.push(current.clone());
                return;
            }

            let candidate = candidates[i];

            current.push(candidate);
            backtrack(current, i, sum + candidate, candidates, target, result);
            current.pop().expect("pop");
            backtrack(current, i + 1, sum, candidates, target, result);
        }

        let mut result = vec![];
        let mut current = vec![];

        backtrack(&mut current, 0, 0, &candidates, target, &mut result);

        result
    }
}

fn main() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7],]
    );
    assert_eq!(
        Solution::combination_sum(vec![8, 7, 4, 3], 11),
        vec![vec![8, 3], vec![7, 4], vec![4, 4, 3],]
    );
}
