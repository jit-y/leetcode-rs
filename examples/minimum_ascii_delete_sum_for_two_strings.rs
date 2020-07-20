struct Solution {}
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for i in (0..s1.len()).rev() {
            dp[i][s2.len()] = dp[i + 1][s2.len()] + s1[i] as i32;
        }

        for i in (0..s2.len()).rev() {
            dp[s1.len()][i] = dp[s1.len()][i + 1] + s2[i] as i32;
        }

        for i in (0..s1.len()).rev() {
            for j in (0..s2.len()).rev() {
                if s1[i] == s2[j] {
                    dp[i][j] = dp[i + 1][j + 1];
                } else {
                    dp[i][j] =
                        std::cmp::min(dp[i + 1][j] + s1[i] as i32, dp[i][j + 1] + s2[j] as i32);
                }
            }
        }

        dp[0][0]
    }
}

fn main() {
    //   0,  0,  0,313 s
    //   0,  0,  0,198 e
    //   0,  0,  0, 97 a
    // 314,213,116,  0
    //   e   a   t
    assert_eq!(
        Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()),
        231
    );
    assert_eq!(
        Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()),
        403
    );
}
