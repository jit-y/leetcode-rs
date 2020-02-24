struct Solution {}
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let s = s.chars().collect::<Vec<char>>();
        let t = t.chars().collect::<Vec<char>>();

        let mut s_m = std::collections::HashMap::new();
        let mut t_m = std::collections::HashMap::new();

        for i in 0..s.len() {
            s_m.insert(s[i], t[i]);
            t_m.insert(t[i], s[i]);
        }

        for i in 0..s.len() {
            if s_m.get(&s[i]) != t.get(i) || t_m.get(&t[i]) != s.get(i) {
                return false;
            }
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
