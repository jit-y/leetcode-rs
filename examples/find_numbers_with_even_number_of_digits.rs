struct Solution {}
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for n in nums {
            let mut n = n;
            let mut digit = 0;

            while n > 0 {
                digit += 1;

                n /= 10;
            }

            if digit % 2 == 0 {
                result += 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
}
