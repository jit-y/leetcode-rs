struct Solution {}
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = Vec::new();

        if row_index < 0 {
            return result;
        }

        result.push(1);
        if row_index == 0 {
            return result;
        }

        result.push(1);
        if row_index == 1 {
            return result;
        }

        for _ in 2..=row_index {
            let mut current = vec![1, 1];
            for j in 0..(result.len() - 1) {
                current.insert(1, result[j] + result[j + 1]);
            }

            result = current.clone();
        }

        result
    }

    pub fn get_row_2(row_index: i32) -> Vec<i32> {
        let mut result = vec![0; row_index as usize + 1];
        result[0] = 1;

        for i in 0..(row_index + 1) {
            for j in (1..=i).rev() {
                result[j as usize] += result[j as usize - 1];
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
    assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
    assert_eq!(Solution::get_row(5), vec![1, 5, 10, 10, 5, 1]);
    assert_eq!(Solution::get_row_2(2), vec![1, 2, 1]);
    assert_eq!(Solution::get_row_2(3), vec![1, 3, 3, 1]);
    assert_eq!(Solution::get_row_2(4), vec![1, 4, 6, 4, 1]);
    assert_eq!(Solution::get_row_2(5), vec![1, 5, 10, 10, 5, 1]);
}
