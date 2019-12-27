struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let bytes = s.as_bytes();

        if bytes.len() % 2 != 0 {
            return false;
        }

        for v in s.as_bytes().iter() {
            match v {
                b'(' | b'{' | b'[' => stack.push(v),
                b')' => match stack.pop() {
                    None => return false,
                    Some(l) => {
                        if *l != b'(' {
                            return false;
                        }
                    }
                },
                b'}' => match stack.pop() {
                    None => return false,
                    Some(l) => {
                        if *l != b'{' {
                            return false;
                        }
                    }
                },
                b']' => match stack.pop() {
                    None => return false,
                    Some(l) => {
                        if *l != b'[' {
                            return false;
                        }
                    }
                },
                _ => return false,
            }
        }

        stack.is_empty()
    }
}
