struct Solution {}
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        (0..start_time.len()).fold(0, |sum, i| {
            sum + if start_time[i] <= query_time && query_time <= end_time[i] {
                1
            } else {
                0
            }
        })
    }
}

fn main() {
    assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
    assert_eq!(Solution::busy_student(vec![4], vec![4], 4), 1);
    assert_eq!(Solution::busy_student(vec![4], vec![4], 5), 0);
    assert_eq!(
        Solution::busy_student(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7),
        0
    );
    assert_eq!(
        Solution::busy_student(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![10, 10, 10, 10, 10, 10, 10, 10, 10],
            5
        ),
        5
    );
}
