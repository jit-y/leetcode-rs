struct Solution {}
impl Solution {
    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let row_len = m.len();
        let mut result = 0;

        let mut visited = vec![false; row_len];

        for i in 0..row_len {
            if visited[i] {
                continue;
            }

            Self::dfs(i, &m, &mut visited);

            result += 1;
        }

        result
    }

    fn dfs(i: usize, m: &Vec<Vec<i32>>, visited: &mut Vec<bool>) {
        visited[i as usize] = true;

        for j in 0..m[i].len() {
            if m[i][j] != 1 || visited[j] || i == j {
                continue;
            }

            Self::dfs(j, m, visited);
        }
    }
}

fn main() {
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1]]),
        1
    );
    assert_eq!(
        Solution::find_circle_num(vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1]
        ]),
        1
    );
}
