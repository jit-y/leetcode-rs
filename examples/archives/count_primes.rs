struct Solution {}
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 1 {
            return 0;
        }

        let l = (n as f32).sqrt().ceil() as i32;
        let mut map = std::collections::HashMap::new();

        map.insert(1, true);
        map.insert(n, true);

        for p in 2..=l {
            let mut x = p;
            x += p;
            while x <= n {
                map.insert(x, true);
                x += p;
            }
        }

        n - map.len() as i32
    }
}

fn main() {
    assert_eq!(Solution::count_primes(10), 4);
    assert_eq!(Solution::count_primes(4), 2);
    assert_eq!(Solution::count_primes(2), 0);
}
