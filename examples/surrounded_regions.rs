struct Solution {}
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn dfs(
            board: &mut Vec<Vec<char>>,
            visited: &mut Vec<Vec<bool>>,
            x: usize,
            y: usize,
            row_len: usize,
            col_len: usize,
        ) -> bool {
            if visited[x][y] {
                return true;
            }

            if x <= 0 || x >= row_len - 1 || y <= 0 || y >= row_len - 1 {
                return false;
            }

            visited[x][y] = true;
            let mut up = true;
            let mut right = true;
            let mut down = true;
            let mut left = true;

            if board[x][y - 1] != 'X' {
                up = dfs(board, visited, x, y - 1, row_len, col_len);
            }

            if board[x + 1][y] != 'X' {
                right = dfs(board, visited, x + 1, y, row_len, col_len);
            }

            if board[x][y + 1] != 'X' {
                down = dfs(board, visited, x, y + 1, row_len, col_len);
            }

            if board[x - 1][y] != 'X' {
                left = dfs(board, visited, x - 1, y, row_len, col_len);
            }

            if up && right && down && left {
                board[x][y] = 'X';
            }

            true
        }

        if board.is_empty() {
            return;
        }

        let row_len = board.len();
        let col_len = board[0].len();
        let mut visited = vec![vec![false; col_len]; row_len];
        let mut board = board;

        for x in 0..row_len {
            for y in 0..col_len {
                if board[x][y] == 'O' {
                    if dfs(&mut board, &mut visited, x, y, row_len, col_len) {
                        board[x][y] = 'X';
                    }
                }
            }
        }
    }
}

fn main() {
    let mut a = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    Solution::solve(&mut a);
    assert_eq!(
        a,
        vec![
            vec!['X', 'X', 'X', 'X',],
            vec!['X', 'X', 'X', 'X',],
            vec!['X', 'X', 'X', 'X',],
            vec!['X', 'O', 'X', 'X',],
        ]
    );
    let mut a = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'O'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
    ];
    Solution::solve(&mut a);
    assert_eq!(
        a,
        vec![
            vec!['X', 'X', 'X', 'X',],
            vec!['X', 'X', 'X', 'O',],
            vec!['X', 'X', 'X', 'X',],
            vec!['X', 'X', 'X', 'X',],
        ]
    );
    let mut a = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
    ];
    Solution::solve(&mut a);
    assert_eq!(
        a,
        vec![
            vec!['X', 'X', 'X', 'X',],
            vec!['X', 'X', 'X', 'X',],
            vec!['X', 'X', 'X', 'X',],
            vec!['X', 'X', 'X', 'X',],
        ]
    );
    let mut a = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'O'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
    ];
    Solution::solve(&mut a);
    assert_eq!(
        a,
        vec![
            vec!['X', 'X', 'X', 'X',],
            vec!['X', 'O', 'O', 'O',],
            vec!['X', 'X', 'X', 'X',],
            vec!['X', 'X', 'X', 'X',],
        ]
    );
}
