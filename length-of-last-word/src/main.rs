struct Solution {}

impl Solution {
    pub fn length_of_last_word(string: String) -> i32 {
        match string.trim().split(" ").last() {
            Some(v) => v.len() as i32,
            None => 0,
        }
    }
}

fn main() {
    let string = "   fly me   to   the moon  ".to_string();
    let result = Solution::length_of_last_word(string);
    println!("{result}");
}
