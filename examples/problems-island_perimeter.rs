struct Solution {}
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let row_len = grid.len();
        for i in 0..row_len {
            let col_len = grid[i].len();
            for j in 0..col_len {
                if grid[i][j] == 0 {
                    continue;
                }
                let mut v = 4;

                if i > 0 && grid[i - 1][j] == 1 {
                    v -= 1;
                }

                if i < (row_len - 1) && grid[i + 1][j] == 1 {
                    v -= 1;
                }

                if j > 0 && grid[i][j - 1] == 1 {
                    v -= 1;
                }

                if j < (col_len - 1) && grid[i][j + 1] == 1 {
                    v -= 1;
                }

                result += v;
            }
        }

        result
    }

    pub fn island_perimeter_2(grid: Vec<Vec<i32>>) -> i32 {
        let mut c = 0;
        let mut neighbor = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    continue;
                }

                c += 1;

                if i > 0 && grid[i - 1][j] == 1 {
                    neighbor += 1;
                }

                if j > 0 && grid[i][j - 1] == 1 {
                    neighbor += 1;
                }
            }
        }

        c * 4 - neighbor * 2
    }
}

fn main() {
    let v = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];

    assert_eq!(Solution::island_perimeter(v.clone()), 16);
    assert_eq!(Solution::island_perimeter_2(v.clone()), 16);
}
