struct Solution {}
impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut h = a.len() - 1;

        while l < h {
            let m = l + (h - l) / 2;

            if a[m] > a[m + 1] {
                h = m;
            } else {
                l = m + 1;
            }
        }

        l as i32
    }
}

fn main() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    assert_eq!(
        Solution::peak_index_in_mountain_array(vec![0, 3, 2, 1, 0]),
        1
    );
}
