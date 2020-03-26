struct Solution {}
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut up_down: i32 = 0;
        let mut right_left: i32 = 0;

        moves.chars().for_each(|c| match c {
            'U' => up_down += 1,
            'D' => up_down -= 1,
            'R' => right_left += 1,
            'L' => right_left -= 1,
            _ => {}
        });

        up_down == 0 && right_left == 0
    }
}

fn main() {
    assert_eq!(Solution::judge_circle("UD".to_string()), true);
    assert_eq!(Solution::judge_circle("LL".to_string()), false);
    assert_eq!(Solution::judge_circle("UDXX".to_string()), true);
}
