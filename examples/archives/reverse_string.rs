struct Solution {}
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() <= 1 {
            return;
        }
        let mut start = 0;
        let mut end = s.len() - 1;

        while start < end {
            s.swap(start, end);
            start += 1;
            end -= 1;
        }
    }

    pub fn reverse_string_2(s: &mut Vec<char>) {
        if s.is_empty() {
            return;
        }

        let len = s.len();
        let mid = len / 2;

        for i in 0..mid {
            s.swap(i, len - i - 1);
        }
    }

    pub fn reverse_string_std(s: &mut Vec<char>) {
        s.reverse();
    }
}

fn main() {
    let mut a = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut a);
    assert_eq!(a, vec!['o', 'l', 'l', 'e', 'h']);

    let mut b = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    Solution::reverse_string(&mut b);
    assert_eq!(b, vec!['h', 'a', 'n', 'n', 'a', 'H']);

    let mut c = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string_std(&mut c);
    assert_eq!(c, vec!['o', 'l', 'l', 'e', 'h']);

    let mut d = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    Solution::reverse_string(&mut d);
    assert_eq!(d, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}
