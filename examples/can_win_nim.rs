struct Solution {}
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

fn main() {
    assert_eq!(Solution::can_win_nim(4), false);
}
