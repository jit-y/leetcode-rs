struct Solution {}
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut a = HashMap::new();
        let mut b = HashMap::new();

        for i in 0..nums.len() {
            map.entry(nums[i]).and_modify(|n| *n += 1).or_insert(1);
            a.entry(nums[i]).or_insert(i);
            b.insert(nums[i], i);
        }

        let max_digree = map.iter().max_by_key(|v| v.1).expect("max").1;
        let mut result = nums.len() as i32;

        for n in nums {
            if map.get(&n).expect("get") == max_digree {
                let a_val = *a.get(&n).expect("a") as i32;
                let b_val = *b.get(&n).expect("b") as i32;

                result = std::cmp::min(result, b_val - a_val + 1);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
    assert_eq!(
        Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]),
        6
    );
}
