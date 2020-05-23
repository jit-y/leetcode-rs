struct Solution {}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        match haystack.find(needle.as_str()) {
            None => -1,
            Some(n) => n as i32,
        }
    }

    pub fn str_str2(haystack: String, needle: String) -> i32 {
        if haystack == needle {
            return 0;
        }
        let needle = needle.chars().collect::<Vec<char>>();
        let head = match needle.get(0) {
            None => return 0,
            Some(n) => n,
        };

        let haystack = haystack.chars().collect::<Vec<char>>();
        let mut result = -1;

        for (i, c) in haystack.iter().enumerate() {
            if c == head {
                if needle
                    .iter()
                    .skip(0)
                    .enumerate()
                    .all(|(j, n)| match haystack.get(i + j) {
                        None => false,
                        Some(h) => h == n,
                    })
                {
                    result = i as i32;
                    break;
                }
            }
        }

        result
    }
}

fn main() {
    println!(
        "{}",
        Solution::str_str("hello".to_string(), "ll".to_string())
    );
    println!(
        "{}",
        Solution::str_str("aaaaa".to_string(), "bba".to_string())
    );
    println!("{}", Solution::str_str("".to_string(), "".to_string()));
    println!("{}", Solution::str_str("a".to_string(), "".to_string()));
}
