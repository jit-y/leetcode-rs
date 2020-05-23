struct Solution {}
impl Solution {
    pub fn sum_even_after_queries(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let mut a = a;

        for query in queries.iter() {
            let tmp = a[query[1] as usize];
            a[query[1] as usize] += query[0];

            let v = a
                .iter()
                .fold(0, |acc, n| if n % 2 == 0 { acc + n } else { acc });

            result.push(v);
        }

        result
    }

    pub fn sum_even_after_queries_2(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let mut a = a;
        let mut sum = a
            .iter()
            .fold(0, |acc, n| if n % 2 == 0 { acc + n } else { acc });

        for query in queries.iter() {
            let val = query[0];
            let idx = query[1] as usize;

            if a[idx] % 2 == 0 {
                sum -= a[idx];
            }

            a[idx] += val;

            if a[idx] % 2 == 0 {
                sum += a[idx];
            }

            result.push(sum);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::sum_even_after_queries(
            vec![1, 2, 3, 4],
            vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]
        ),
        vec![8, 6, 2, 4]
    );
}
