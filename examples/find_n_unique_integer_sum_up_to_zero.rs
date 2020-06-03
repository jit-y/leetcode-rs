struct Solution {}
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let h = n / 2;
        let mut result = vec![];

        if n % 2 == 1 {
            result.push(0);
        }

        for i in 1..=h {
            result.push(i as i32);
        }

        for i in 1..=h {
            result.insert(0, -(i as i32));
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::sum_zero(3).iter().sum::<i32>(), 0);
    assert_eq!(Solution::sum_zero(5).iter().sum::<i32>(), 0);
    assert_eq!(Solution::sum_zero(100).iter().sum::<i32>(), 0);
    assert_eq!(Solution::sum_zero(1000).iter().sum::<i32>(), 0);
}
