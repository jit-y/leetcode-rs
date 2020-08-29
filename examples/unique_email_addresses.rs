struct Solution {}
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let mut set = HashSet::new();

        for email in emails {
            let email: Vec<&str> = email.split('@').collect();
            let local = email[0];
            let domain = email[1];

            let local: Vec<&str> = local.split('+').collect();
            let mut v = String::new();
            for c in local[0].chars() {
                match c {
                    '.' => {}
                    _ => v.push(c),
                }
            }

            v.push('@');
            v.push_str(domain);

            set.insert(v);
        }

        set.len() as i32
    }
}

fn main() {
    assert_eq!(
        Solution::num_unique_emails(
            vec![
                "test.email+alex@leetcode.com",
                "test.e.mail+bob.cathy@leetcode.com",
                "testemail+david@lee.tcode.com"
            ]
            .into_iter()
            .map(|s| s.to_owned())
            .collect()
        ),
        2
    );
}
