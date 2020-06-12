struct Solution {}
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut result = vec![];
        let max = candies.iter().max().unwrap();

        for c in candies.iter() {
            let v = c + extra_candies >= *max;

            result.push(v);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
        vec![true, true, true, false, true]
    );
    assert_eq!(
        Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
        vec![true, false, false, false, false]
    );
    assert_eq!(
        Solution::kids_with_candies(vec![12, 1, 12], 10),
        vec![true, false, true]
    );
}
