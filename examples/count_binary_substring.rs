struct Solution {}
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut result = 0;
        let cs = s.chars().collect::<Vec<char>>();
        let mut arr = vec![0, 0];
        arr[cs[0].to_digit(10).unwrap() as usize] += 1;

        for i in 1..cs.len() {
            let prev = cs[i - 1].to_digit(10).unwrap() as usize;
            let current = cs[i].to_digit(10).unwrap() as usize;
            let other = current ^ 1;
            arr[current] += 1;

            if prev != current {
                result += 1;
                arr[current] = 1;
                continue;
            }

            if arr[current] == arr[other] {
                result += 1;
                arr[other] = 0;
                continue;
            }

            if arr[other] >= arr[current] {
                result += 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6);
    assert_eq!(
        Solution::count_binary_substrings("000111000".to_string()),
        6
    );
    assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
    assert_eq!(Solution::count_binary_substrings("00100".to_string()), 2);
    assert_eq!(Solution::count_binary_substrings("11110011".to_string()), 4);
    assert_eq!(
        Solution::count_binary_substrings("100111001".to_string()),
        6
    );
}
