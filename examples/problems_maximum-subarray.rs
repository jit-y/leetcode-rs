struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current = match nums.first() {
            None => return 0,
            Some(x) => *x,
        };
        let mut result = current;

        for v in nums.into_iter().skip(1) {
            current = std::cmp::max(v, current + v);

            if current > result {
                result = current;
            }
        }

        result
    }
}

fn main() {
    let a = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

    println!("{}", Solution::max_sub_array(a));
}
