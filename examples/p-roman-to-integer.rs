struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result: i32 = 0;
        let word_iter = s.split("");
        let words: Vec<&str> = word_iter.clone().collect();

        for (i, v) in word_iter.enumerate() {
            let next = words.get(i + 1);
            match v {
                "" => continue,
                "I" => {
                    if let Some(v) = next {
                        if *v == "V" || *v == "X" {
                            result -= 1;
                            continue;
                        }
                    }

                    result += 1;
                }
                "V" => {
                    result += 5;
                }
                "X" => {
                    if let Some(v) = next {
                        if *v == "L" || *v == "C" {
                            result -= 10;
                            continue;
                        }
                    }

                    result += 10;
                }
                "L" => {
                    result += 50;
                }
                "C" => {
                    if let Some(v) = next {
                        if *v == "D" || *v == "M" {
                            result -= 100;
                            continue;
                        }
                    }

                    result += 100;
                }
                "D" => {
                    result += 500;
                }
                "M" => {
                    result += 1000;
                }
                _ => unreachable!(),
            }
        }

        result
    }
}

fn main() {}
