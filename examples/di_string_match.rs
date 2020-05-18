struct Solution {}
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut i = 0;
        let mut d = s.len() as i32;
        let mut result = vec![];

        for c in s.chars() {
            match c {
                'I' => {
                    result.push(i);
                    i += 1;
                }
                'D' => {
                    result.push(d);
                    d -= 1;
                }
                _ => unreachable!(),
            }
        }

        if i > d {
            result.push(i);
        } else {
            result.push(d);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::di_string_match("IDID".to_string()),
        vec![0, 4, 1, 3, 2]
    );
}
