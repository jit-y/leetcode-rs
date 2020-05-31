struct Solution {}

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let len = time.len();
        let mut result = 0;

        for i in 0..len {
            let t = time[i];

            for j in (i + 1)..len {
                if (t + time[j]) % 60 == 0 {
                    result += 1;
                }
            }
        }

        result
    }

    pub fn num_pairs_divisible_by60_2(time: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut result = 0;

        for i in 0..60 {
            map.insert(i as i32, 0);
        }

        for t in time {
            let m = t % 60;
            let g = 60 - m;

            if let Some(v) = map.get(&(g % 60)) {
                result += v;
            }

            map.entry(m).and_modify(|v| *v += 1);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]),
        3
    );
    assert_eq!(Solution::num_pairs_divisible_by60(vec![60, 60, 60]), 3);
    assert_eq!(
        Solution::num_pairs_divisible_by60(vec![
            418, 204, 77, 278, 239, 457, 284, 263, 372, 279, 476, 416, 360, 18
        ]),
        1
    );
    assert_eq!(
        Solution::num_pairs_divisible_by60_2(vec![30, 20, 150, 100, 40]),
        3
    );
    assert_eq!(Solution::num_pairs_divisible_by60_2(vec![60, 60, 60]), 3);
    assert_eq!(
        Solution::num_pairs_divisible_by60_2(vec![
            418, 204, 77, 278, 239, 457, 284, 263, 372, 279, 476, 416, 360, 18
        ]),
        1
    );
}
