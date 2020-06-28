struct Solution {}
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = vec![];

        for v in 1..=n {
            if v % 15 == 0 {
                result.push("FizzBuzz".to_string());
                continue;
            }

            if v % 3 == 0 {
                result.push("Fizz".to_string());
                continue;
            }

            if v % 5 == 0 {
                result.push("Buzz".to_string());
                continue;
            }

            result.push(v.to_string())
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::fizz_buzz(15),
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    )
}
