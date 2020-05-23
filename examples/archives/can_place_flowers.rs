struct Solution {}
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if flowerbed.len() > 0 && n == 0 {
            return true;
        }

        let mut flowerbed = flowerbed;
        let mut i = 0;
        let mut count = 0;

        while i < flowerbed.len() {
            if flowerbed[i] != 0 {
                i += 1;
                continue;
            }

            if (i == 0 || flowerbed[i - 1] == 0)
                && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
            {
                flowerbed[i] = 1;
                count += 1;
                i += 1;
            }

            if count >= n {
                return true;
            }

            i += 1;
        }

        false
    }
}

fn main() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    assert_eq!(Solution::can_place_flowers(vec![1], 0), true);
}
