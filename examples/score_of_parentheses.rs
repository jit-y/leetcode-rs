struct Solution {}
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        use std::collections::VecDeque;
        let mut stack = VecDeque::new();

        stack.push_back(0);

        for c in s.chars() {
            match c {
                '(' => stack.push_back(0),
                ')' => {
                    let a = stack.pop_back().expect("a");
                    let b = stack.pop_back().expect("b");

                    let v = b + std::cmp::max(a * 2, 1);

                    stack.push_back(v);
                }
                _ => unreachable!(),
            }
        }

        stack.pop_back().expect("must be an answer")
    }

    pub fn score_of_parentheses_2(s: String) -> i32 {
        fn divide_and_conquer(s: Vec<char>, from: usize, to: usize) -> i32 {
            let mut result = 0;
            let mut balance = 0;
            let mut from = from;

            let mut cur = from;

            while cur < to {
                let v = match s.get(cur) {
                    Some('(') => 1,
                    Some(')') => -1,
                    _ => unimplemented!(),
                };

                balance += v;

                if balance != 0 {
                    cur += 1;
                    continue;
                }

                if cur - from == 1 {
                    result += 1;
                } else {
                    result += 2 * divide_and_conquer(s.clone(), from + 1, cur);
                }

                from = cur + 1;
                cur = from;
            }

            result
        }

        let s: Vec<char> = s.chars().collect();
        let l = s.len();
        divide_and_conquer(s, 0, l)
    }
}

fn main() {
    assert_eq!(Solution::score_of_parentheses("()".to_owned()), 1);
    assert_eq!(Solution::score_of_parentheses("()()".to_owned()), 2);
    assert_eq!(Solution::score_of_parentheses("(())".to_owned()), 2);
    assert_eq!(Solution::score_of_parentheses("(()(()))".to_owned()), 6);

    assert_eq!(Solution::score_of_parentheses_2("()".to_owned()), 1);
    assert_eq!(Solution::score_of_parentheses_2("()()".to_owned()), 2);
    assert_eq!(Solution::score_of_parentheses_2("(())".to_owned()), 2);
    assert_eq!(Solution::score_of_parentheses_2("(()(()))".to_owned()), 6);
}
