struct Solution {}
impl Solution {
    pub fn to_lower_case(str: String) -> String {
        let mut result = String::new();
        let code_ua = 'A' as u32;
        let code_uz = 'Z' as u32;
        let code_a = 'a' as u32;
        let gap_upper_to_lower = code_a - code_uz;

        for c in str.chars() {
            let mut code = c as u32;
            if code_ua <= code && code <= code_uz {
                code += gap_upper_to_lower + (code - code_ua) + (code_uz - code);
                result.push(std::char::from_u32(code).expect("push"));
                continue;
            }

            result.push(c);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::to_lower_case("Hello".to_string()),
        "hello".to_string()
    );
    assert_eq!(
        Solution::to_lower_case("here".to_string()),
        "here".to_string()
    );
    assert_eq!(
        Solution::to_lower_case("LOVELY".to_string()),
        "lovely".to_string()
    );
    assert_eq!(
        Solution::to_lower_case("al&phaBET".to_string()),
        "al&phabet".to_string()
    );
}
