struct Solution {}
impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;

        let mut stack = VecDeque::new();
        let mut result = vec![];

        for i in (0..t.len()).rev() {
            while !stack.is_empty() && t[i] >= t[*stack.back().expect("back")] {
                stack.pop_back();
            }

            match stack.back() {
                None => result.insert(0, 0),
                Some(n) => {
                    result.insert(0, (*n - i) as i32);
                }
            }
            stack.push_back(i);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![55, 38, 53, 81, 61, 93, 97, 32, 43, 78]),
        vec![3, 1, 1, 2, 1, 1, 0, 1, 1, 0]
    );
}
