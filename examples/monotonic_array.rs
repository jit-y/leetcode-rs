struct Solution {}
impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut inc = true;
        let mut dec = true;

        for i in 1..a.len() {
            if a[i - 1] == a[i] {
                continue;
            }

            if a[i - 1] < a[i] {
                dec = false;
            }

            if a[i - 1] > a[i] {
                inc = false;
            }
        }

        inc || dec
    }
}

fn main() {
    assert_eq!(Solution::is_monotonic(vec![1, 2, 2, 3]), true);
    assert_eq!(Solution::is_monotonic(vec![6, 5, 4, 4]), true);
    assert_eq!(Solution::is_monotonic(vec![1, 3, 2]), false);
}
