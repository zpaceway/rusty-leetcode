struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 0;
        let mut result = digits.to_vec();
        result.reverse();
        for (i, digit) in result.to_vec().iter().enumerate() {
            let mut value = digit + carry;
            if i == 0 {
                value = digit + 1 + carry;
            }
            carry = value / 10;
            if value / 10 > 0 {
                value = value % 10;
            }
            result[i] = value;
        }

        if carry > 0 {
            result.extend(vec![carry]);
        }

        result.reverse();

        return result;
    }
}

fn main() {
    let digits = vec![9];
    let result = Solution::plus_one(digits);
    println!("{:?}", result);
}
