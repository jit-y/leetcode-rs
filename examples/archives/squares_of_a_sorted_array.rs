struct Solution {}
impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;

        a.sort_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap());

        a.into_iter().map(|v| v * v).collect()
    }
}
fn main() {
    assert_eq!(
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
}
