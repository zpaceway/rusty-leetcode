struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let result: Vec<i32> = nums.iter().filter(|x| **x != val).map(|v| *v).collect();
        nums.clear();
        nums.extend(result);

        return nums.len() as i32;
    }
}

fn main() {
    let mut nums: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    Solution::remove_element(&mut nums, val);
}
