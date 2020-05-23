struct Solution {}
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            grid: &mut Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            x: usize,
            y: usize,
            row_len: usize,
            col_len: usize,
        ) -> i32 {
            visited[x as usize][y as usize] = true;

            let mut len = 1;

            if x > 0 && !visited[x - 1][y] && grid[x - 1][y] == 1 {
                len += dfs(grid, visited, x - 1, y, row_len, col_len);
            }

            if x < (row_len - 1) && !visited[x + 1][y] && grid[x + 1][y] == 1 {
                len += dfs(grid, visited, x + 1, y, row_len, col_len);
            }

            if y > 0 && !visited[x][y - 1] && grid[x][y - 1] == 1 {
                len += dfs(grid, visited, x, y - 1, row_len, col_len);
            }

            if y < (col_len - 1) && !visited[x][y + 1] && grid[x][y + 1] == 1 {
                len += dfs(grid, visited, x, y + 1, row_len, col_len);
            }

            len
        }

        if grid.is_empty() {
            return 0;
        }

        let mut grid = grid;
        let row_len = grid.len();
        let col_len = grid[0].len();
        let mut visited = vec![vec![false; col_len]; row_len];
        let mut ones = Vec::new();

        for i in 0..row_len {
            for j in 0..col_len {
                if grid[i][j] == 1 {
                    ones.push((i, j));
                }
            }
        }
        let mut result = 0;

        for (x, y) in ones {
            result = std::cmp::max(result, dfs(&mut grid, &mut visited, x, y, row_len, col_len));
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ]),
        6
    )
}
