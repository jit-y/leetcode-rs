struct Solution {}
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows <= 0 {
            return vec![];
        }

        let mut result = Vec::new();
        result.push(vec![1]);
        if num_rows == 1 {
            return result;
        }

        result.push(vec![1, 1]);
        if num_rows == 2 {
            return result;
        }

        for i in 3..=num_rows {
            let mut init = vec![1, 1];
            let prev = result.get((i - 2) as usize).expect("get");
            for j in 0..(prev.len() - 1) {
                init.insert(1, prev[j] + prev[j + 1]);
            }

            result.push(init);
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::generate(1), vec![vec![1]]);
    assert_eq!(Solution::generate(2), vec![vec![1], vec![1, 1]]);
    assert_eq!(
        Solution::generate(3),
        vec![vec![1], vec![1, 1], vec![1, 2, 1]]
    );
    assert_eq!(
        Solution::generate(4),
        vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]
    );
    assert_eq!(
        Solution::generate(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]
    );
}
