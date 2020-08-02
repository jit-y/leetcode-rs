struct Solution {}
impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        let mut result = vec![];

        for (i, word) in s.split_ascii_whitespace().enumerate() {
            let mut w = String::new();
            let mut chars = word.chars();
            let c = chars.next().expect("char");

            for c in chars {
                w.push(c);
            }

            let l = c.to_ascii_lowercase();
            match l {
                'a' => w.insert(0, c),
                'e' => w.insert(0, c),
                'i' => w.insert(0, c),
                'o' => w.insert(0, c),
                'u' => w.insert(0, c),
                _ => {
                    w.push(c);
                }
            }

            w.push_str("ma");
            for _ in 0..(i + 1) {
                w.push('a');
            }

            result.push(w);
        }

        result.join(" ")
    }
}

fn main() {
    assert_eq!(
        Solution::to_goat_latin("I speak Goat Latin".to_string()),
        "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string()
    );
    assert_eq!(
        Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()),
        "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string()
    );
}
