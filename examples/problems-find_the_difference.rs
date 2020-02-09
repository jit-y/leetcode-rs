struct Solution {}
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut t_sum = 0;

        for c in t.chars() {
            t_sum += c as u32;
        }

        for c in s.chars() {
            t_sum -= c as u32;
        }

        std::char::from_u32(t_sum).expect("something wrong")
    }
}

fn main() {
    assert_eq!(
        Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
        'e'
    );
}
