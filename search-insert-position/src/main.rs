struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut position = -1;
        let mut possible_possition = -1;
        let mut iteration = 0;
        for val in nums.iter() {
            if *val == target {
                position = iteration.clone();
            }
            if target < *val && possible_possition == -1 {
                possible_possition = iteration.clone();
            }
            iteration += 1;
        }

        if position != -1 {
            return position;
        } else if possible_possition != -1 {
            return possible_possition;
        } else {
            return iteration;
        };
    }
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 7;
    let result = Solution::search_insert(nums, target);
    println!("{result}");
}
