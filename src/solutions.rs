pub struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, n1) in nums.iter().enumerate() {
            for (j, n2) in nums.iter().enumerate() {
                if *n1 + *n2 == target && i != j {
                    return Vec::from([i as i32, j as i32]);
                }
            }
        }
        Vec::from([0, 0])
    }
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i: usize = 0;
        while i < nums.len() {
            match nums.get(i + 1) {
                Some(next) => if next == &nums[i] { nums.remove(i + 1); } else { i += 1; }
                None => i += 1
            }
        }
        nums.len() as i32
    }
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total = 0;
        for day in 0..prices.len() {
            match prices.get(day + 1) {
                Some(next_price) => if next_price > &prices[day] {
                    total += next_price - prices[day]
                }
                None => {}
            }
        }
        total as i32
    }
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        nums.rotate_right(k.abs() as usize % len);
    }
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        for (i, n1) in nums.iter().enumerate() {
            for (j, n2) in nums.iter().enumerate() {
                if i != j && n1 == n2 {
                    return true
                }
            }
        }
        false
    }
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut num_count: HashMap<i32, usize> = HashMap::new();
        for n in nums.iter() {
            match num_count.get_mut(n) {
                Some(n) => *n += 1,
                None => { num_count.insert(*n, 1); }
            }
        }
        for (n, times) in num_count.iter() {
            if *times == 1 {
                return *n
            }
        }
        0
    }
}