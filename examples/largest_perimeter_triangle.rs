struct Solution {}
impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        let mut a = a;

        a.sort();

        for i in (0..a.len() - 2).rev() {
            if a[i] + a[i + 1] > a[i + 2] {
                return a[i] + a[i + 1] + a[i + 2];
            }
        }

        0
    }
}

fn main() {
    assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
    assert_eq!(Solution::largest_perimeter(vec![3, 2, 3, 4]), 10);
}
