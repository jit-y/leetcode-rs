struct Solution {}
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.len() < 1 {
            return s;
        }

        let mut s = s;
        let len = s.len();
        let mut left = 0;
        let mut right = len - 1;

        while left < right {
            let l = s.remove(left);
            match l {
                'a' | 'i' | 'u' | 'e' | 'o' | 'A' | 'I' | 'U' | 'E' | 'O' => {}
                _ => {
                    s.insert(left, l);
                    left += 1;
                    continue;
                }
            }

            let r = s.remove(right - 1);
            match r {
                'a' | 'i' | 'u' | 'e' | 'o' | 'A' | 'I' | 'U' | 'E' | 'O' => {}
                _ => {
                    s.insert(left, l);
                    s.insert(right, r);
                    right -= 1;
                    continue;
                }
            }

            s.insert(left, r);
            s.insert(right, l);

            left += 1;
            right -= 1;
        }

        s
    }
}

fn main() {
    assert_eq!(
        Solution::reverse_vowels("hello".to_string()),
        "holle".to_string()
    );
    assert_eq!(
        Solution::reverse_vowels("leetcode".to_string()),
        "leotcede".to_string()
    );
}
