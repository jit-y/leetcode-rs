struct Solution {}
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut dic = std::collections::HashMap::new();
        for (i, c) in order.chars().enumerate() {
            dic.insert(c, i);
        }

        for i in 1..words.len() {
            let w1: Vec<char> = (&words[i - 1]).chars().collect();
            let w2: Vec<char> = (&words[i]).chars().collect();
            let l = if w1.len() < w2.len() {
                w1.len()
            } else {
                w2.len()
            };

            let mut halted = false;
            for j in 0..l {
                if w1[j] == w2[j] {
                    halted = true;
                    continue;
                }

                let w1i = dic.get(&w1[j]).unwrap();
                let w2i = dic.get(&w2[j]).unwrap();

                if w1i > w2i {
                    return false;
                }

                break;
            }

            if halted {
                if w1.len() > w2.len() {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    assert_eq!(
        Solution::is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ),
        true
    );
    assert_eq!(
        Solution::is_alien_sorted(
            vec!["word".to_string(), "world".to_string(), "row".to_string()],
            "worldabcefghijkmnpqstuvxyz".to_string()
        ),
        false
    );
    assert_eq!(
        Solution::is_alien_sorted(
            vec!["kuvp".to_string(), "q".to_string()],
            "ngxlkthsjuoqcpavbfdermiywz".to_string()
        ),
        true
    );
}
