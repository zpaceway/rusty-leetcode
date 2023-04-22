struct Solution {}

use std::{collections::HashSet, vec};

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let nums_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut new_nums: Vec<i32> = nums_set.iter().cloned().collect();
        new_nums.sort();
        nums.clear();
        nums.extend(new_nums);
        return nums_set.len() as i32;
    }
}

fn main() {
    let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result = Solution::remove_duplicates(&mut nums);
    println!("{result}");
}
