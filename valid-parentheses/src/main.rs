struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn is_valid(input_string: String) -> bool {
        if input_string.len() <= 1 {
            return false;
        }

        let stacks_mapping: HashMap<String, String> = HashMap::from([
            (")".to_string(), "(".to_string()),
            ("]".to_string(), "[".to_string()),
            ("}".to_string(), "{".to_string()),
        ]);
        let mut ordered_stack: Vec<String> = vec![];

        for input_char in input_string.chars() {
            let input_string = input_char.to_string();
            let default = "".to_string();
            let opening_operator_mapping = stacks_mapping.get(&input_string).unwrap_or(&default);

            if !opening_operator_mapping.is_empty() {
                if ordered_stack.len() == 0 {
                    return false;
                }
                let last_stack = ordered_stack.pop().unwrap();
                if last_stack == *opening_operator_mapping {
                    continue;
                } else {
                    return false;
                }
            }
            ordered_stack.push(input_string.clone());
        }

        return ordered_stack.len() == 0;
    }
}

fn main() {
    let result = Solution::is_valid("(){}(([]))".to_string());
    println!("{result}");
}
