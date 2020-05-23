struct Solution {}
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }

        let mut a = s.chars().collect::<Vec<char>>();

        let mut l = 0;
        let mut r = a.len() - 1;

        while l < r {
            if a[l].is_alphabetic() && a[r].is_alphabetic() {
                a.swap(l, r);
                l += 1;
                r -= 1;
            }

            if !a[l].is_alphabetic() {
                l += 1;
            }

            if !a[r].is_alphabetic() {
                r -= 1;
            }
        }

        a.into_iter().collect::<String>()
    }
}

fn main() {
    assert_eq!(
        Solution::reverse_only_letters("ab-cd".to_string()),
        "dc-ba".to_string()
    );
    assert_eq!(
        Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
        "j-Ih-gfE-dCba".to_string()
    );
    assert_eq!(
        Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
        "Qedo1ct-eeLg=ntse-T!".to_string()
    );
    assert_eq!(
        Solution::reverse_only_letters("".to_string()),
        "".to_string()
    );
    assert_eq!(
        Solution::reverse_only_letters("a".to_string()),
        "a".to_string()
    );
}
