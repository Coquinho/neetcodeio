use problem_00704_binary_search;

fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let flattened: Vec<i32> = matrix.into_iter().flatten().collect();

        let found = problem_00704_binary_search::_search(
            &flattened, 0, flattened.len(), target
        );

        found != -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case01() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1 ,3 ,5 ,7],
                     vec![10,11,16,20],
                     vec![23,30,34,60]],
            3), 
            true
        );
    }

    #[test]
    fn test_case02() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1 ,3 ,5 ,7],
                     vec![10,11,16,20],
                     vec![23,30,34,60]],
            13), 
            false
        );
    }
}
