struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn dfs(
            grid: &mut Vec<Vec<char>>,
            x: usize,
            y: usize,
            row_len: usize,
            col_len: usize,
        ) -> i32 {
            if grid[x][y] == '0' {
                return 0;
            }

            grid[x][y] = '0';
            if x > 0 && grid[x - 1][y] == '1' {
                dfs(grid, x - 1, y, row_len, col_len);
            }

            if x < (row_len - 1) && grid[x + 1][y] == '1' {
                dfs(grid, x + 1, y, row_len, col_len);
            }

            if y > 0 && grid[x][y - 1] == '1' {
                dfs(grid, x, y - 1, row_len, col_len);
            }

            if y < col_len - 1 && grid[x][y + 1] == '1' {
                dfs(grid, x, y + 1, row_len, col_len);
            }

            1
        }

        if grid.is_empty() {
            return 0;
        }

        let row_len = grid.len();
        let col_len = grid[0].len();
        let mut ones = vec![];

        for i in 0..row_len {
            for j in 0..col_len {
                if grid[i][j] == '1' {
                    ones.push((i, j));
                }
            }
        }

        let mut grid = grid;
        ones.into_iter().fold(0, |acc, (x, y)| {
            acc + dfs(&mut grid, x, y, row_len, col_len)
        })
    }
}

fn main() {
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ]),
        1
    );
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ]),
        3
    );
}
