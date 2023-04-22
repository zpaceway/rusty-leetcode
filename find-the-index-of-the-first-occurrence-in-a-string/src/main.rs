struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let position = haystack.find(&needle);
        return match position {
            Some(v) => v as i32,
            None => -1,
        };
    }
}

fn main() {
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();
    let result = Solution::str_str(haystack, needle);
    println!("{result}");
}
