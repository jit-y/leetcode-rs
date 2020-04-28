struct Solution {}
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;

        while stones.len() > 1 {
            stones.sort();
            let a = stones.pop().expect("a");
            let b = stones.last_mut().expect("b");

            if &a == b {
                stones.pop();
            } else {
                *b = a - *b;
            }
        }

        stones.pop().unwrap_or(0)
    }
}

fn main() {
    assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1,);
    assert_eq!(Solution::last_stone_weight(vec![1, 3]), 2);
}
