use std::vec;

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut num = x;
        let mut arr: Vec<i32> = vec![];
        while num > 0 {
            let digit = num % 10;
            arr.push(digit);
            num /= 10;
        }

        let mut reversed = arr.clone();
        reversed.reverse();

        return arr == reversed;
    }
}

fn main() {
    let result = Solution::is_palindrome(121);

    println!("{result}");
}
