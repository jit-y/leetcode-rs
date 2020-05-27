struct Solution {}
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        let mut points = points;
        let mut current = points.remove(0);
        let mut seconds = 0;

        for point in points {
            let a = (current[0] - point[0]).abs();
            let b = (current[1] - point[1]).abs();

            seconds += std::cmp::max(a, b);
            current = point;
        }

        seconds
    }
}

fn main() {
    assert_eq!(
        Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
        7
    );
    assert_eq!(
        Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
        5
    );
}
