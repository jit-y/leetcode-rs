struct Solution {}
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::new();
        let mut b = 0;

        for paren in s.chars() {
            if paren == ')' {
                b -= 1;
            }

            if b != 0 {
                result.push(paren);
            }

            if paren == '(' {
                b += 1;
            }
        }

        result
    }

    pub fn remove_outer_parentheses_2(s: String) -> String {
        let mut stack = vec![];
        let mut result = vec![];

        for paren in s.chars() {
            match paren {
                '(' => {
                    stack.push(paren);
                }
                ')' => {
                    if let Some(v) = stack.pop() {
                        if stack.is_empty() {
                            continue;
                        }

                        result.push(v);

                        while stack.len() > 1 {
                            result.push(stack.pop().unwrap());
                        }

                        result.push(paren);

                        continue;
                    }

                    result.push(paren);
                }
                _ => unreachable!(),
            }
        }

        result.into_iter().collect()
    }
}

fn main() {
    assert_eq!(
        Solution::remove_outer_parentheses("(()())(())".to_string()),
        "()()()".to_string()
    );
    assert_eq!(
        Solution::remove_outer_parentheses("()()()".to_string()),
        "".to_string()
    );
    assert_eq!(
        Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
        "()()()()(())".to_string()
    );
    assert_eq!(
        Solution::remove_outer_parentheses("((()())(()()))".to_string()),
        "(()())(()())".to_string()
    );
}
