struct Solution {}
impl Solution {
    pub fn fib(n: i32) -> i32 {
        use std::collections::HashMap;
        fn _fib(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if n <= 1 {
                return n;
            }

            if let Some(v) = memo.get(&n) {
                return *v;
            }

            let v = _fib(n - 1, memo) + _fib(n - 2, memo);
            memo.insert(n, v);

            v
        }

        let mut memo = HashMap::new();

        _fib(n, &mut memo)
    }
}

fn main() {
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(3), 2);
    assert_eq!(Solution::fib(30), 832040);
}
