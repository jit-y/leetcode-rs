struct Solution {}
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut result = 0f64;

        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                for k in (j + 1)..points.len() {
                    let a = &points[i];
                    let b = &points[j];
                    let c = &points[k];

                    let x = a[0] * b[1] + b[0] * c[1] + c[0] * a[1];
                    let y = a[1] * b[0] + b[1] * c[0] + c[1] * a[0];

                    let v = ((x - y) as f64).abs() * 0.5;

                    result = result.max(v);
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::largest_triangle_area(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![0, 2],
            vec![2, 0]
        ]),
        2.0
    );
}
