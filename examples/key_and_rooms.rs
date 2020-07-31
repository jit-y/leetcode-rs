struct Solution {}
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        use std::collections::HashSet;

        fn visit(rooms: &Vec<Vec<i32>>, visited: &mut HashSet<usize>, current: usize) {
            visited.insert(current);

            let keys = &rooms[current];

            for k in keys {
                let key = *k as usize;
                if visited.contains(&key) {
                    continue;
                }

                visit(rooms, visited, key);
            }
        }

        let mut visited = HashSet::new();
        visit(&rooms, &mut visited, 0);

        rooms.len() == visited.len()
    }
}

fn main() {
    assert_eq!(
        Solution::can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]),
        true
    );
    assert_eq!(
        Solution::can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]),
        false
    );
}
