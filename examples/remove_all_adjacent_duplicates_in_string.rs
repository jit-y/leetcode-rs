struct Solution {}
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = vec![];

        for c in s.chars() {
            if let None = stack.last() {
                stack.push(c);
                continue;
            }

            let v = stack.last().expect("last");
            if c == *v {
                stack.pop();
                continue;
            }

            stack.push(c);
        }

        stack.into_iter().collect()
    }
}

fn main() {
    assert_eq!(
        Solution::remove_duplicates("abbaca".to_string()),
        "ca".to_string()
    );
}
