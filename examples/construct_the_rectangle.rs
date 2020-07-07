struct Solution {}
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut w = (area as f32).sqrt() as i32;

        while w >= 1 {
            if area % w == 0 {
                return vec![area / w, w];
            }

            w -= 1;
        }

        unreachable!()
    }
}

fn main() {
    assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
    assert_eq!(Solution::construct_rectangle(3456), vec![64, 54]);
}
