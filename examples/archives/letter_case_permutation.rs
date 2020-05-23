struct Solution {}
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        fn permutation(s: Vec<char>, idx: usize, result: &mut Vec<String>) {
            if s.len() == idx {
                result.push(s.iter().collect::<String>());
                return;
            }
            permutation(s.clone(), idx + 1, result);

            if s[idx].is_digit(10) {
                return;
            }

            let mut s = s;
            if s[idx].is_ascii_uppercase() {
                s[idx] = s[idx].to_ascii_lowercase();
            } else {
                s[idx] = s[idx].to_ascii_uppercase();
            }

            permutation(s, idx + 1, result);
        }

        let s = s.chars().collect::<Vec<char>>();
        let mut result = vec![];
        permutation(s, 0, &mut result);

        result
    }
}

fn main() {
    assert_eq!(
        Solution::letter_case_permutation("a1b2".to_string()),
        vec!["a1b2", "a1B2", "A1b2", "A1B2"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::letter_case_permutation("3z4".to_string()),
        vec!["3z4", "3Z4"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::letter_case_permutation("12345".to_string()),
        vec!["12345"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::letter_case_permutation("C".to_string()),
        vec!["C", "c"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
}
