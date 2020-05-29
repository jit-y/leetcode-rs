struct Solution {}
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        arr.sort();

        arr.sort_by_cached_key(|v| {
            let mut n = 0;
            let mut v = *v;
            while v > 0 {
                n += v & 1;
                v = v >> 1;
            }

            n
        });

        arr
    }
}

fn main() {
    assert_eq!(
        Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
    );
    assert_eq!(
        Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
        vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
    );
    assert_eq!(
        Solution::sort_by_bits(vec![10000, 10000]),
        vec![10000, 10000]
    );
    assert_eq!(
        Solution::sort_by_bits(vec![2, 3, 5, 7, 11, 13, 17, 19]),
        vec![2, 3, 5, 17, 7, 11, 13, 19]
    );
    assert_eq!(
        Solution::sort_by_bits(vec![10, 100, 1000, 10000]),
        vec![10, 100, 10000, 1000]
    );
}
