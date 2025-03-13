use std::cmp::Ordering::{Equal, Greater, Less};

pub fn _search(nums: &[i32], start: usize, end: usize, target: i32) -> i32 {
    let len = end - start;
    if len < 1 {
        return -1;
    }

    let midle = len / 2 + start;
    let value = nums[midle];
    if len == 1 {
        return if value == target { 0 } else { -1 };
    }
    println!("nums {:?}", nums);
    println!("midle: {}, value: {}", midle, value);

    match value.cmp(&target) {
        Equal => return midle as i32,
        Greater => return _search(nums, start, midle, target),
        Less => return _search(nums, midle, end, target),
    }
}

struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        if len <= 0 {
            return -1;
        }

        _search(&nums, 0, len, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_find() {
        assert_eq!(Solution::search(vec![0], 0), 0);
    }

    #[test]
    fn test_should_not_find_on_empty_list() {
        assert_eq!(Solution::search(vec![], 0), -1);
    }

    #[test]
    fn test_should_not_find_on_trivial_list() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn test_should_find_in_list() {
        for i in 0..7 {
            println!("i: {}", i);
            assert_eq!(Solution::search(vec![0, 1, 2, 3, 4, 5, 6, 7], i), i);
        }
    }

    #[test]
    fn test_should_not_find_in_list() {
        for _i in 0..7 {
            assert_eq!(Solution::search(vec![0, 1, 2, 3, 4, 5, 6, 7], 10), -1);
        }
    }
}
