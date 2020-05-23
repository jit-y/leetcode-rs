struct Solution {}
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut idxs = vec![];
        let mut result = vec![];
        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                idxs.push(i as i32);
            }
        }

        for (i, _) in s.chars().enumerate() {
            let mut v = 10_000;
            for idx in idxs.iter() {
                v = std::cmp::min(v, (idx - i as i32).abs());
                if v == 0 {
                    break;
                }
            }

            result.push(v);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
    );
}
