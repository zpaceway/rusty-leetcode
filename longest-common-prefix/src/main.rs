struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common: Vec<String> = vec!["".to_string()];
        let mut iterations = 0;
        loop {
            let mut should_stop = false;
            let new_common: Vec<String> = strs
                .iter()
                .map(|v| {
                    if iterations + 1 > v.len() {
                        should_stop = true;
                        return &v[0..iterations];
                    } else {
                        return &v[0..iterations + 1];
                    }
                })
                .collect::<HashSet<&str>>()
                .iter()
                .map(|s| s.to_string())
                .collect();
            if should_stop {
                break;
            }
            iterations += 1;
            if new_common.len() == 1 {
                common = new_common;
            } else {
                break;
            }
        }

        return common[0].clone();
    }
}

fn main() {
    let r = Solution::longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);
    println!("{r}");
}
