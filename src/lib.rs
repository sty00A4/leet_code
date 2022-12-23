#![allow(unused)]
mod solutions;
use solutions::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number() {
        assert_eq!(Solution::single_number(vec![2,2,1]), 1);
        assert_eq!(Solution::single_number(vec![4,1,2,1,2]), 4);
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
    #[test]
    fn contains_duplicate() {
        assert!(Solution::contains_duplicate(vec![1,2,3,1]));
        assert!(!Solution::contains_duplicate(vec![1,2,3,4]));
        assert!(Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));
    }
    #[test]
    fn rotate_array() {
        let mut v = vec![1,2,3,4,5,6,7];
        Solution::rotate(&mut v,  3);
        assert_eq!(v, vec![5,6,7,1,2,3,4]);
        let mut v = vec![-1,-100,3,99];
        Solution::rotate(&mut v,  2);
        assert_eq!(v, vec![3,99,-1,-100]);
        let mut v = vec![-1,];
        Solution::rotate(&mut v,  2);
        assert_eq!(v, vec![-1]);
    }
    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 7);
        assert_eq!(Solution::max_profit(vec![1,2,3,4,5]),   4);
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]),   0);
    }
    #[test]
    fn two_sum() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3,2,4],     6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3,3],       6), vec![0, 1]);
    }
}
