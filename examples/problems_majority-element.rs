struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();

        for x in nums {
            map.entry(x).and_modify(|y| *y += 1).or_insert(1);
        }

        map.into_iter()
            .max_by(|(_, xval), (_, yval)| xval.cmp(yval))
            .expect("something wrong")
            .0
    }

    pub fn majority_element_bm(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = None;

        for v in nums {
            if count == 0 {
                candidate = Some(v);
            }

            if let Some(c) = candidate {
                if c == v {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
        }

        candidate.expect("something wrong")
    }
}

fn main() {
    let a = vec![3, 2, 3];
    let b = vec![2, 2, 1, 1, 1, 2, 2];
    let c = vec![3, 2, 3];
    let d = vec![2, 2, 1, 1, 1, 2, 2];

    assert_eq!(Solution::majority_element(a), 3);
    assert_eq!(Solution::majority_element(b), 2);
    assert_eq!(Solution::majority_element_bm(c), 3);
    assert_eq!(Solution::majority_element_bm(d), 2);
}
