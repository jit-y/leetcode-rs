struct Solution {}
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut change = HashMap::new();

        for b in bills {
            match b {
                val @ 5 => {
                    change.entry(val).and_modify(|v| *v += 1).or_insert(1);
                }
                val @ 10 => match change.get_mut(&5) {
                    None => return false,
                    Some(n) => {
                        *n -= 1;
                        change.entry(val).and_modify(|v| *v += 1).or_insert(1);
                    }
                },
                20 => {
                    let d5 = *change.get(&5).unwrap_or(&mut 0);
                    let d10 = *change.get(&10).unwrap_or(&mut 0);

                    if d10 >= 1 && d5 >= 1 {
                        change.entry(10).and_modify(|v| *v -= 1);
                        change.entry(5).and_modify(|v| *v -= 1);

                        continue;
                    }

                    if d5 >= 3 {
                        change.entry(5).and_modify(|v| *v -= 3);

                        continue;
                    }

                    return false;
                }
                _ => unreachable!(),
            }
        }

        true
    }
}

fn main() {
    assert_eq!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]), true);
    assert_eq!(Solution::lemonade_change(vec![5, 5, 10]), true);
    assert_eq!(Solution::lemonade_change(vec![10, 10]), false);
    assert_eq!(Solution::lemonade_change(vec![5, 5, 10, 10, 20]), false);
    assert_eq!(
        Solution::lemonade_change(vec![
            5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5
        ]),
        true
    );
}
