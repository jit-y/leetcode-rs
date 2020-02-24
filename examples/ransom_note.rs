struct Solution {}
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut rn = std::collections::HashMap::new();
        for c in ransom_note.chars().into_iter() {
            *rn.entry(c).or_insert(0) += 1;
        }

        for c in magazine.chars().into_iter() {
            match rn.get_mut(&c) {
                None => continue,
                Some(v) => {
                    *v -= 1;
                    if *v == 0 {
                        rn.remove(&c);
                    }
                }
            }
        }

        rn.is_empty()
    }

    pub fn can_construct_2(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut arr = vec![0; 256];
        for c in magazine.chars().into_iter() {
            arr[c as usize] += 1;
        }

        for c in ransom_note.chars().into_iter() {
            if arr[c as usize] == 0 {
                return false;
            }

            arr[c as usize] -= 1;
        }

        true
    }
}

fn main() {
    assert_eq!(
        Solution::can_construct("aa".to_string(), "aab".to_string()),
        true
    );
    assert_eq!(
        Solution::can_construct_2("aa".to_string(), "aab".to_string()),
        true
    );
}
