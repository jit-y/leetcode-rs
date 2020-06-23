struct Solution {}
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        use std::collections::HashMap;
        let mut bulls = 0;
        let mut cows = 0;

        let secret = secret.chars().collect::<Vec<char>>();
        let guess = guess.chars().collect::<Vec<char>>();
        let mut bulls_map = vec![false; secret.len()];
        let mut cows_map = HashMap::new();

        for i in 0..secret.len() {
            if secret[i] == guess[i] {
                bulls += 1;
                bulls_map[i] = true;
            } else {
                *cows_map.entry(secret[i]).or_insert(0) += 1;
            }
        }

        for (i, c) in guess.iter().enumerate() {
            if bulls_map[i] {
                continue;
            }

            if let Some(v) = cows_map.get_mut(c) {
                if *v <= 0 {
                    continue;
                }

                cows += 1;
                *v -= 1;
                if *v <= 0 {
                    cows_map.remove(c);
                }
            }
        }

        format!("{}A{}B", bulls, cows)
    }
}

fn main() {
    assert_eq!(
        Solution::get_hint("1807".to_string(), "7810".to_string()),
        "1A3B".to_string()
    );
    assert_eq!(
        Solution::get_hint("1123".to_string(), "0111".to_string()),
        "1A1B".to_string()
    );
    assert_eq!(
        Solution::get_hint("11".to_string(), "01".to_string()),
        "1A0B".to_string()
    );
}
