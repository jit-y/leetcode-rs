struct Solution {}
impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let mut odd = 1;

        for even in (0..a.len()).step_by(2) {
            if a[even] % 2 == 0 {
                continue;
            }

            while a[odd] % 2 == 1 {
                odd += 2;
            }

            a.swap(even, odd);
        }

        a
    }
}

fn main() {
    assert_eq!(
        Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
        vec![4, 5, 2, 7]
    );
    assert_eq!(
        Solution::sort_array_by_parity_ii(vec![3, 1, 4, 2]),
        vec![2, 1, 4, 3]
    );
    assert_eq!(
        Solution::sort_array_by_parity_ii(vec![1, 0, 1, 2, 1, 0, 2, 1]),
        vec![0, 1, 2, 1, 0, 1, 2, 1]
    );
}
