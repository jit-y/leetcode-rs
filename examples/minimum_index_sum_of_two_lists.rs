struct Solution {}
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        for (i, el) in list1.into_iter().enumerate() {
            map.insert(el, i);
        }

        let mut result = vec![];
        let mut min_idx = 10000;

        for (i, el) in list2.into_iter().enumerate() {
            if let Some(j) = map.get(&el) {
                if min_idx > i + j {
                    result.clear();
                    result.push(el);
                    min_idx = i + j;

                    continue;
                }

                if min_idx == i + j {
                    result.push(el);
                }
            }
        }

        result
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
