struct Solution {}
impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let mut l = 0;
        let mut r = a.len() - 1;

        while l < r {
            if a[l] % 2 > a[r] % 2 {
                a.swap(l, r);
            }

            if a[l] % 2 == 0 {
                l += 1;
            }

            if a[r] % 2 == 1 {
                r -= 1;
            }
        }

        a
    }
}

fn main() {
    assert_eq!(
        Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
        vec![4, 2, 1, 3]
    )
}
