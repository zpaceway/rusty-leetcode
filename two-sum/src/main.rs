use std::vec;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num1) in nums.iter().enumerate() {
            for (j, num2) in nums.iter().enumerate() {
                if j == i {
                    continue;
                }
                if num1 + num2 == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        return vec![-1, -1];
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result); // Output: [0, 1]
}
