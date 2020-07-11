struct Solution {}
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        for (i, el) in list1.into_iter().enumerate() {
            map.insert(el, i);
        }

        let mut result: HashMap<usize, Vec<String>> = HashMap::new();
        let mut min_idx = 10000;

        for (i, el) in list2.into_iter().enumerate() {
            if let Some(j) = map.get(&el) {
                min_idx = std::cmp::min(min_idx, i + j);
                match result.get_mut(&(i + j)) {
                    Some(arr) => {
                        arr.push(el);
                    }
                    None => {
                        let mut n = vec![];
                        n.push(el);

                        result.insert(i + j, n);
                    }
                }
            }
        }

        result.remove(&min_idx).unwrap_or(vec![])
    }
}

fn main() {
    assert_eq!(
        Solution::find_restaurant(
            vec!["Shogun", "Tapioca Express", "Burger King", "KFC"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            vec![
                "Piatti",
                "The Grill at Torrey Pines",
                "Hungry Hunter Steakhouse",
                "Shogun"
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        ),
        vec!["Shogun".to_string()]
    );
    assert_eq!(
        Solution::find_restaurant(
            vec!["Shogun", "Tapioca Express", "Burger King", "KFC"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            vec![
                "Piatti",
                "The Grill at Torrey Pines",
                "Hungry Hunter Steakhouse",
                "Shogun"
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        ),
        vec!["Shogun".to_string()]
    );
}
