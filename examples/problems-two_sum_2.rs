struct Solution {}
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        let mut result = Vec::new();

        while left < right {
            let val = numbers[left] + numbers[right];
            if val == target {
                result.push((left + 1) as i32);
                result.push((right + 1) as i32);

                break;
            }

            if val > target {
                right -= 1;
            } else {
                left += 1;
            }
        }

        result
    }

    // bad solution
    pub fn two_sum_dp(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();

        let mut dp = Vec::with_capacity((target + 1) as usize);

        for _ in 0..target + 1 {
            dp.push(None);
        }

        dp[0].get_or_insert(0);

        for n in numbers.iter() {
            for current in (0..target).rev() {
                if let Some(_) = dp[current as usize] {
                    let a = n + current as i32;

                    if a <= target {
                        dp[a as usize].get_or_insert(*n);
                    }
                }
            }

            if dp[target as usize].is_some() {
                break;
            }
        }

        let mut next_index = target;

        for (i, n) in numbers.iter().enumerate().rev() {
            if next_index == 0 {
                break;
            }

            let next = dp[next_index as usize].unwrap();
            if n != &next {
                continue;
            }

            result.insert(0, (i + 1) as i32);

            next_index -= next;
        }

        result
    }
}

fn main() {
    let a = vec![2, 3, 4];
    let b = vec![2, 7, 11, 15];
    println!("{:?}", Solution::two_sum(a, 6));
    println!("{:?}", Solution::two_sum(b, 9));
}
