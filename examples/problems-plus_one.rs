struct Solution {}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let size = digits.len();
        let mut carry = None;

        let last = match digits.get_mut(size - 1) {
            None => return digits,
            Some(v) => {
                *v += 1;
                v
            }
        };

        if *last >= 10 {
            carry = Some(true);
            *last = 0;
        }

        for d in digits.iter_mut().rev().skip(1) {
            match carry {
                None => break,
                Some(_) => {
                    *d += 1;
                    if *d < 10 {
                        carry = None;
                        break;
                    }

                    carry = Some(true);
                    *d = 0;
                }
            }
        }

        if carry.is_some() {
            digits.insert(0, 1);
        }

        digits
    }
}

fn main() {
    println!("{:?}", Solution::plus_one(vec![1, 2, 3]));
    println!("{:?}", Solution::plus_one(vec![9, 9, 9]));
    println!("{:?}", Solution::plus_one(vec![0]));
}
