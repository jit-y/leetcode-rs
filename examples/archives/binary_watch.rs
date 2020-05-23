struct Solution {}
impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut result = Vec::new();
        for h in 0..=11u32 {
            for m in 0..=59u32 {
                let n = ((h << 6) | m).count_ones();
                if n == (num as u32) {
                    let m = if m < 10 {
                        format!("0{}", m)
                    } else {
                        m.to_string()
                    };
                    result.push(format!("{}:{}", h, m));
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::read_binary_watch(1),
        vec!["0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
}
