struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let len = s.len();
        let mut result = 0;

        // 0: [[x]]xxxx
        // 1: [x][x]xxx
        // 2: x[[x]]xxx
        // 3: x[x][x]xx
        // ...
        for n in 0..=len * 2 - 1 {
            let mut l = (n / 2) as i32;
            let mut r = l + n as i32 % 2;

            while l >= 0 && r < len as i32 && s[l as usize] == s[r as usize] {
                result += 1;
                l -= 1;
                r += 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
}
