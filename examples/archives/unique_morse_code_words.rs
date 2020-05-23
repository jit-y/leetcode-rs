struct Solution {}
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let dic = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut map = std::collections::HashMap::new();
        let code_a = 'a' as u32;

        for word in words {
            let mut s = String::new();
            for c in word.chars() {
                let m = dic[(c as u32 - code_a) as usize];
                s.push_str(m);
            }

            map.insert(s, true);
        }

        map.len() as i32
    }
}

fn main() {
    assert_eq!(
        Solution::unique_morse_representations(vec![
            "gin".to_string(),
            "zen".to_string(),
            "gig".to_string(),
            "msg".to_string()
        ]),
        2
    )
}
