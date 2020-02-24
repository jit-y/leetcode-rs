struct Solution {}
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut result: Option<i32> = None;
        for (i, x) in nums.iter().enumerate() {
            for y in nums.iter().skip(i + 1) {
                if x == y {
                    result = Some(*x);
                    break;
                }
            }

            if result.is_some() {
                break;
            }
        }

        result.expect("Something wrong")
    }

    pub fn find_duplicate_floyd(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0];
        let mut fast = nums[nums[0] as usize];

        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
        }

        fast = 0;

        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }

        slow
    }
}

fn main() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 5, 2, 2]), 2);
    assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    assert_eq!(Solution::find_duplicate(vec![2, 2, 2, 2, 2, 2]), 2);
    assert_eq!(Solution::find_duplicate_floyd(vec![1, 3, 4, 5, 2, 2]), 2);
    assert_eq!(Solution::find_duplicate_floyd(vec![3, 1, 3, 4, 2]), 3);
    assert_eq!(Solution::find_duplicate_floyd(vec![2, 2, 2, 2, 2, 2]), 2);
}
