struct Solution {}
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut memo = std::collections::HashMap::new();

        for n in nums.into_iter() {
            match memo.get_mut(&n) {
                Some(_) => return true,
                None => {
                    memo.insert(n, true);
                }
            }
        }

        false
    }
}

fn main() {
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(
        Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
        true
    );
}
