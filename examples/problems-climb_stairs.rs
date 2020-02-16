struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }

        if n == 1 || n == 2 {
            return n;
        }

        let mut one_before = 2;
        let mut two_before = 1;
        let mut all = 0;

        for _ in 2..n {
            all = one_before + two_before;
            two_before = one_before;
            one_before = all;
        }

        all
    }
}

fn main() {
    assert_eq!(Solution::climb_stairs(1), 1);
    assert_eq!(Solution::climb_stairs(2), 2);
    assert_eq!(Solution::climb_stairs(3), 3);
    assert_eq!(Solution::climb_stairs(4), 5);
    assert_eq!(Solution::climb_stairs(5), 8);
}
