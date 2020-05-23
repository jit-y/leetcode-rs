struct Solution {}
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for n in left..=right {
            if n % 10 == 0 {
                continue;
            }

            let mut current = n;

            while current > 0 {
                let div = current % 10;

                if div == 0 || n % div != 0 {
                    break;
                }
                current /= 10;
            }

            if current == 0 {
                result.push(n);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::self_dividing_numbers(1, 22),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
}
