fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let max = *piles.iter().max().unwrap();
        let min = 1;

        Solution::binary_search(&piles, h, min, max)
    }

    pub fn binary_search(piles: &[i32], h: i32, min: i32, max: i32) -> i32 {
        let mut min = min;
        let mut max = max;

        while min < max  {
            let middle = (max - min)/2 + min;
            let can_eat = Solution::can_eat_in_time(
                &piles, h, middle
            );

            if can_eat {
                max = middle;
            } else {
                min = middle + 1;
            }
        }

        min
    }

    fn can_eat_in_time(
        piles: &[i32], h: i32, k: i32
    ) -> bool {
        let mut hours = 0;
        for &pile in piles {
            hours += (pile + k - 1)/k;

            let cant_eat = hours > h;
            if cant_eat {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case01() {
        assert_eq!(
            Solution::min_eating_speed(
                vec![3,6,7,11], 8
            ),
            4
        );
    }

    #[test]
    fn test_case02() {
        assert_eq!(
            Solution::min_eating_speed(
                vec![30,11,23,4,20], 5
            ),
            30
        );
    }

    #[test]
    fn test_case03() {
        assert_eq!(
            Solution::min_eating_speed(
                vec![30,11,23,4,20], 6
            ),
            23
        );
    }

    #[test]
    fn test_case04() {
        assert_eq!(
            Solution::min_eating_speed(
                vec![312884470], 312884469
            ),
            2
        );
    }

    #[test]
    fn test_case05() {
        assert_eq!(
            Solution::min_eating_speed(
                vec![805306368,805306368,805306368], 1000000000
            ),
            3 
        );
    }

    #[test]
    fn test_case06() {
        assert_eq!(
            Solution::min_eating_speed(
                vec![312884470], 968709470
            ),
            1
        );
    }
}
