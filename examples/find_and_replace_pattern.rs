struct Solution {}
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        fn is_match(word: &Vec<char>, pattern: &Vec<char>) -> bool {
            use std::collections::HashMap;

            let mut m1 = HashMap::new();
            let mut m2 = HashMap::new();

            for i in 0..word.len() {
                let c = word[i];
                let p = pattern[i];

                m1.entry(c).or_insert(p);
                m2.entry(p).or_insert(c);

                if m1.get(&c).expect("m1") != &p {
                    return false;
                }

                if m2.get(&p).expect("m2") != &c {
                    return false;
                }
            }

            true
        }

        let pattern: Vec<char> = pattern.chars().collect();
        let mut result = vec![];

        for word in words {
            let word: Vec<char> = word.chars().collect();
            if is_match(&word, &pattern) {
                result.push(word.into_iter().collect());
            }
        }

        result
    }

    pub fn find_and_replace_pattern_2(words: Vec<String>, pattern: String) -> Vec<String> {
        fn is_match(word: &Vec<char>, pattern: &Vec<char>) -> bool {
            use std::collections::HashMap;

            let mut m1 = HashMap::new();
            let mut m2 = HashMap::new();

            for i in 0..word.len() {
                let c = word[i];
                let p = pattern[i];

                m1.insert(c, p);
                m2.insert(p, c);
            }

            for i in 0..word.len() {
                let c = word[i];
                let p = pattern[i];

                if m1.get(&c).expect("m1") != &p {
                    return false;
                }

                if m2.get(&p).expect("m2") != &c {
                    return false;
                }
            }

            true
        }

        let pattern: Vec<char> = pattern.chars().collect();
        let mut result = vec![];

        for word in words {
            let word: Vec<char> = word.chars().collect();
            if is_match(&word, &pattern) {
                result.push(word.into_iter().collect());
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::find_and_replace_pattern(
            vec!["abc", "deq", "mee", "aqq", "dkd", "ccc"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            "abb".to_string()
        ),
        vec!["mee", "aqq"]
    );
    assert_eq!(
        Solution::find_and_replace_pattern_2(
            vec!["abc", "deq", "mee", "aqq", "dkd", "ccc"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            "abb".to_string()
        ),
        vec!["mee", "aqq"]
    );
}
