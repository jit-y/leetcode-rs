struct Solution {}
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for cpdomain in cpdomains {
            let mut v = cpdomain.split_ascii_whitespace();
            let n = v.next().unwrap().parse::<i32>().unwrap();
            let mut names = v.next().unwrap().split('.').collect::<Vec<&str>>();

            while !names.is_empty() {
                let domain = names.join(".");
                map.entry(domain).and_modify(|val| *val += n).or_insert(n);
                names.remove(0);
            }
        }

        let mut result = vec![];

        for (domain, n) in map {
            let v = format!("{} {}", n, domain);
            result.push(v);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".to_string()]),
        vec![
            "9001 discuss.leetcode.com".to_string(),
            "9001 leetcode.com".to_string(),
            "9001 com".to_string()
        ]
    );
}
