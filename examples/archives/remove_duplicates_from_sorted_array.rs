struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut prev: Option<i32> = None;

        for _ in 0..nums.len() {
            let n = nums.remove(0);

            match prev {
                Some(v) => {
                    if n != v {
                        nums.push(n);
                    }
                }
                None => nums.push(n),
            }

            prev = Some(n);
        }

        nums.len() as i32
    }
}

fn main() {
    let mut v = vec![1, 1, 2, 3, 4, 4, 4, 5];

    let res = Solution::remove_duplicates(&mut v);

    println!("{}, {:?}", res, v);
}
