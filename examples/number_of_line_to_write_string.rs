struct Solution {}
impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let a = 'a' as usize;
        let mut rest = 0;
        let mut num = 1;

        for c in s.chars() {
            let v = widths[c as usize - a];

            if rest + v > 100 {
                rest = v;
                num += 1;
            } else {
                rest += v;
            }
        }

        vec![num, rest]
    }
}

fn main() {
    assert_eq!(
        Solution::number_of_lines(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ),
        vec![3, 60]
    );
    assert_eq!(
        Solution::number_of_lines(
            vec![
                4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            "bbbcccdddaaa".to_string()
        ),
        vec![2, 4]
    );
}
