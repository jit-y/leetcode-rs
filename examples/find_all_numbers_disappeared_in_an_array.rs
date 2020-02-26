struct Solution {}
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for v in 1..=nums.len() {
            map.insert(v as i32, true);
        }

        for v in nums.into_iter() {
            map.remove(&v);
        }

        let mut result = Vec::new();
        for (k, _) in map {
            result.push(k);
        }

        result
    }

    pub fn find_disappeared_numbers_2(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        for i in 0..nums.len() {
            let v = (nums[i].abs() - 1) as usize;
            if nums[v] > 0 {
                nums[v] = -nums[v];
            }
        }

        let mut result = Vec::new();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                result.push(i as i32 + 1);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
    assert_eq!(
        Solution::find_disappeared_numbers_2(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
}
