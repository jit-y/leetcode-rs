struct Solution {}
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }

        let prev = Solution::count_and_say(n - 1)
            .chars()
            .collect::<Vec<char>>();
        let mut result = String::new();
        let mut n = 1;

        for i in 0..prev.len() {
            let current = prev.get(i).expect("something wrong");
            let next = prev.get(i + 1);

            match next {
                None => {
                    result.push_str(n.to_string().as_str());
                    result.push(*current);
                }
                Some(v) => {
                    if current == v {
                        n += 1;
                    } else {
                        result.push_str(n.to_string().as_str());
                        result.push(*current);
                        n = 1;
                    }
                }
            }
        }

        result
    }
}

fn main() {
    println!("{}", Solution::count_and_say(1));
    println!("{}", Solution::count_and_say(2));
    println!("{}", Solution::count_and_say(3));
    println!("{}", Solution::count_and_say(4));
    println!("{}", Solution::count_and_say(5));
    println!("{}", Solution::count_and_say(6));
    println!("{}", Solution::count_and_say(7));
    println!("{}", Solution::count_and_say(8));
    println!("{}", Solution::count_and_say(9));
    println!("{}", Solution::count_and_say(10));
}
