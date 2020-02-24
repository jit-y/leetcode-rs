struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|n| *n != val);

        nums.len() as i32
    }

    pub fn remove_element_2(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut n = nums.len();
        let mut i = 0;

        while i < n {
            match nums.get(i) {
                None => break,
                Some(v) => {
                    if *v == val {
                        nums.swap(i, n - 1);
                        n -= 1;
                    } else {
                        i += 1;
                    }
                }
            }
        }

        n as i32
    }
}

fn main() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];

    Solution::remove_element(&mut nums, 2);

    println!("{:?}", nums);
}
