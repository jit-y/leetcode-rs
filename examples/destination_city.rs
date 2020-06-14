struct Solution {}
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashMap;
        let mut paths = paths;
        let mut map = HashMap::new();
        let mut result = paths.pop().expect("start array").pop().expect("result");

        for _ in 0..paths.len() {
            let mut a = paths.pop().expect("array");
            let v = a.pop().expect("value");
            let k = a.pop().expect("key");

            map.insert(k, v);
        }

        for _ in 0..map.len() {
            match map.remove(&result) {
                None => break,
                Some(v) => {
                    result = v;
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::dest_city(vec![
            vec!["London".to_string(), "New York".to_string()],
            vec!["New York".to_string(), "Lima".to_string()],
            vec!["Lima".to_string(), "Sao Paulo".to_string()],
        ]),
        "Sao Paulo".to_string()
    );
    assert_eq!(
        Solution::dest_city(vec![
            vec!["B".to_string(), "C".to_string()],
            vec!["D".to_string(), "B".to_string()],
            vec!["C".to_string(), "A".to_string()]
        ]),
        "A".to_string()
    );

    assert_eq!(
        Solution::dest_city(vec![vec!["A".to_string(), "Z".to_string()]]),
        "Z".to_string()
    )
}
