struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, v| acc ^ v)
    }
}

fn main() {
    let a = vec![2, 2, 1];
    let b = vec![4, 1, 2, 1, 2];

    println!("{}", Solution::single_number(a));
    println!("{}", Solution::single_number(b));
}
