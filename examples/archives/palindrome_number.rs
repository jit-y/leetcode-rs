struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut num = x;
        let mut palindromed = 0;

        loop {
            if num <= palindromed {
                break;
            }

            palindromed = palindromed * 10 + num % 10;
            num /= 10;
        }

        // Revert the last half of the number and compare the first half.
        // ex. 1221  =>  21 reveted to 12
        //     12321 =>  321 reverted to 123, and then divided by 10. (== 12)
        num == palindromed || num == palindromed / 10
    }
}

fn main() {}
