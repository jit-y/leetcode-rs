struct Solution {}
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for (i, v) in arr2.into_iter().enumerate() {
            map.insert(v, i);
        }

        let mut arr1 = arr1;
        let mut rest = vec![];
        let mut aidx = arr1.len() - 1;
        let mut current = 0;
        while current <= aidx {
            match map.get(&arr1[current]) {
                None => {
                    rest.push(arr1.remove(current));
                    aidx -= 1;
                }
                Some(_) => current += 1,
            }
        }

        rest.sort();
        arr1.sort_by(|a, b| {
            let a = map.get(a);
            let b = map.get(b);

            a.unwrap().cmp(b.unwrap())
        });
        arr1.append(&mut rest);

        arr1
    }
}

fn main() {
    assert_eq!(
        Solution::relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        ),
        vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
    )
}
