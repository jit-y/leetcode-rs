struct Solution {}
impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut a = a;

        for row in a.iter_mut() {
            let mut left = 0;
            let mut right = row.len() - 1;

            while left < right {
                row.swap(left, right);
                left += 1;
                right -= 1;
            }

            for e in row.iter_mut() {
                *e = !*e & 1;
            }
        }

        a
    }
}

fn main() {
    assert_eq!(
        Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
        vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
    );
    assert_eq!(
        Solution::flip_and_invert_image(vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0]
        ]),
        vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 1, 0]
        ]
    );
}
