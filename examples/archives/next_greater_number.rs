struct Solution {}
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums = nums1;
        let len = nums2.len();

        for i in 0..nums.len() {
            let mut res = -1;
            let x = nums[i];

            for j in 0..len {
                let y = nums2[j];
                if x != y {
                    continue;
                }

                for k in (j + 1)..len {
                    if nums2[k] > x {
                        res = nums2[k];
                        break;
                    }
                }
            }

            nums[i] = res;
        }

        nums
    }

    pub fn next_greater_element_2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums = nums1;
        let mut stack = Vec::new();
        let mut map = std::collections::HashMap::new();

        for n in nums2 {
            while !stack.is_empty() && stack.last().unwrap() < &n {
                let v = stack.pop().unwrap();
                map.insert(v, n);
            }

            stack.push(n);
        }

        for i in 0..nums.len() {
            match map.get(&nums[i]) {
                None => nums[i] = -1,
                Some(v) => nums[i] = *v,
            }
        }

        nums
    }
}

fn main() {
    assert_eq!(
        Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
    assert_eq!(
        Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
    assert_eq!(
        Solution::next_greater_element_2(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
    assert_eq!(
        Solution::next_greater_element_2(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
}
