struct Solution {}

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MAPPING: HashMap<String, i32> = {
        let mut map = HashMap::new();
        map.insert("I".to_string(), 1);
        map.insert("V".to_string(), 5);
        map.insert("X".to_string(), 10);
        map.insert("L".to_string(), 50);
        map.insert("C".to_string(), 100);
        map.insert("D".to_string(), 500);
        map.insert("M".to_string(), 1000);
        map
    };
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;

        for (i, roman_character) in s.chars().enumerate() {
            let current_value = MAPPING.get(&roman_character.to_string());
            let mut next_value = 0;
            if i < s.len() - 1 {
                let next_char = s.chars().nth(i + 1).unwrap().to_string();
                next_value = *(MAPPING.get(&next_char).unwrap());
            }

            match current_value {
                Some(v) => {
                    if next_value > *v {
                        result -= *v;
                    } else {
                        result += *v;
                    }
                }
                None => {}
            }
        }
        return result;
    }
}

fn main() {
    let r = Solution::roman_to_int("IV".to_string());
    println!("{r}");
}
