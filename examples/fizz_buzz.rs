struct Solution {}
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|v| {
                if v % 15 == 0 {
                    "FizzBuzz".to_string()
                } else if v % 3 == 0 {
                    "Fizz".to_string()
                } else if v % 5 == 0 {
                    "Buzz".to_string()
                } else {
                    v.to_string()
                }
            })
            .collect::<Vec<String>>()
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
