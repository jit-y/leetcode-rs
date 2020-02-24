struct Solution {}
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut arr = vec![None; 512];

        let s = s.chars().collect::<Vec<char>>();
        let t = t.chars().collect::<Vec<char>>();

        for i in 0..s.len() {
            if arr[s[i] as usize] != arr[t[i] as usize + 256] {
                return false;
            }

            arr[s[i] as usize] = Some(i);
            arr[t[i] as usize + 256] = Some(i);
        }

        true
    }
}

fn main() {
    assert_eq!(
        Solution::is_isomorphic("egg".to_string(), "add".to_string()),
        true
    );
    assert_eq!(
        Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
        false
    );
    assert_eq!(
        Solution::is_isomorphic("paper".to_string(), "title".to_string()),
        true
    );
}
