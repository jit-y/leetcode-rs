struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 0;

        if let Some(n) = nums.last() {
            dbg!(n);
            if n < &target {
                return nums.len() as i32;
            }
        }

        for (i, n) in nums.into_iter().enumerate() {
            if n > target {
                result = i;
                break;
            }

            if n == target {
                result = i;
                break;
            }
        }

        result as i32
    }
}

fn main() {
    let a = vec![1, 3, 5, 6];
    let b = vec![1, 3, 5, 6];
    let c = vec![1, 3, 5, 6];
    let d = vec![1, 3, 5, 6];

    println!("{}", Solution::search_insert(a, 5));
    println!("{}", Solution::search_insert(b, 2));
    println!("{}", Solution::search_insert(c, 7));
    println!("{}", Solution::search_insert(d, 0));
}
