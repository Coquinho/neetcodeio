fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut left = 0;
        let mut right = nums.len() - 1;

        loop {
            let middle = (right - left) / 2 + left;
            if nums[left] <= nums[middle] && nums[middle] > nums[right] {
                // right has a rotation
                left = middle + 1;
            } else if nums[left] <= nums[middle] && nums[middle] <= nums[right] {
                // already sorted
                break;
            } else if nums[left] > nums[middle] && nums[middle] > nums[right] {
                panicked!(
                    "there is something here l[{}]: {}, m[{}]: {}, r[{}]: {}",
                    left,
                    nums[left],
                    middle,
                    nums[middle],
                    right,
                    nums[right]
                );
            } else if nums[left] > nums[middle] && nums[middle] <= nums[right] {
                // left has a rotation
                right = middle;
            }
        }

        nums[left]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case01() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn test_case02() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn test_case03() {
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn test_case04() {
        assert_eq!(Solution::find_min(vec![1, 2]), 1);
    }

    #[test]
    fn test_case05() {
        assert_eq!(Solution::find_min(vec![2, 1]), 1);
    }
}
