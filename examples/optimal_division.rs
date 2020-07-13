struct Solution {}
impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        if nums.len() == 1 {
            return nums[0].to_string();
        }

        if nums.len() == 2 {
            return format!("{}/{}", nums[0], nums[1]);
        }

        let mut result = String::new();

        result.push_str(&nums[0].to_string());
        result.push('/');
        result.push('(');

        for n in nums.into_iter().skip(1) {
            result.push_str(&n.to_string());
            result.push('/');
        }

        result.pop();
        result.push(')');

        result
    }
}

fn main() {
    assert_eq!(
        Solution::optimal_division(vec![1000, 100, 10, 2]),
        "1000/(100/10/2)".to_string()
    );
}
