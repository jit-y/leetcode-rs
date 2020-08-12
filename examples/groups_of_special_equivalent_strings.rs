struct Solution {}
impl Solution {
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let mut set = HashSet::new();

        for s in a {
            let mut a = vec![0; 52];

            for (i, c) in s.chars().enumerate() {
                let offset = 26 * (i % 2);
                let idx = c as usize - 'a' as usize + offset;

                a[idx] += 1;
            }

            set.insert(a);
        }

        set.len() as i32
    }
}

fn main() {
    assert_eq!(
        Solution::num_special_equiv_groups(
            vec!["abcd", "cdab", "cbad", "xyzz", "zzxy", "zzyx"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        3
    );
}
