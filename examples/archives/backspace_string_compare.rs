struct Solution {}
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        fn process(s: &str) -> Vec<char> {
            let mut stack = vec![];

            for c in s.chars() {
                match c {
                    '#' => {
                        stack.pop();
                    }
                    _ => stack.push(c),
                }
            }

            stack
        }

        process(s.as_str()) == process(t.as_str())
    }
}
fn main() {
    assert_eq!(
        Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("a##c".to_string(), "#a#c".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("a#c".to_string(), "b".to_string()),
        false
    );
}
