struct Solution {}
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn dfs(
            board: &mut Vec<Vec<char>>,
            x: usize,
            y: usize,
            row_len: usize,
            col_len: usize,
        ) -> bool {
            if board[x][y] == 'O' && (x <= 0 || x >= row_len - 1 || y <= 0 || y >= col_len - 1) {
                return false;
            }

            let is_x = board[x][y] == 'X';
            board[x][y] = 'X';

            if (board[x][y - 1] == 'X' || dfs(board, x, y - 1, row_len, col_len))
                && (board[x + 1][y] == 'X' || dfs(board, x + 1, y, row_len, col_len))
                && (board[x][y + 1] == 'X' || dfs(board, x, y + 1, row_len, col_len))
                && (board[x - 1][y] == 'X' || dfs(board, x - 1, y, row_len, col_len))
            {
                if !is_x {
                    board[x][y] = 'O';
                }
                return true;
            }

            if !is_x {
                board[x][y] = 'O';
            }

            false
        }

        if board.is_empty() {
            return;
        }

        let row_len = board.len();
        let col_len = board[0].len();
        let mut board = board;

        for x in 1..row_len {
            for y in 1..col_len {
                if board[x][y] == 'X' {
                    continue;
                }

                if board[x - 1][y] == 'O' && board[x][y - 1] == 'O' {
                    continue;
                }

                if dfs(&mut board, x, y, row_len, col_len) {
                    board[x][y] = 'X';
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
