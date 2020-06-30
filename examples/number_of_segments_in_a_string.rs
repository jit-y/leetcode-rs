struct Solution {}
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_ascii_whitespace().collect::<Vec<&str>>().len() as i32
    }
}

fn main() {
    assert_eq!(
        Solution::count_segments("Hello, my name is John".to_string()),
        5
    );
}
