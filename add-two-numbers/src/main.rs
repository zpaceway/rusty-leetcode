use std::vec;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        head1: Option<Box<ListNode>>,
        head2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut new_head1 = head1.clone();
        let mut new_head2 = head2.clone();
        let mut result: Option<Box<ListNode>> = None;
        let mut vals1: Vec<i32> = vec![];
        let mut vals2: Vec<i32> = vec![];
        let mut raw_vect_result: Vec<i32> = vec![];
        let mut leading: i32 = 0;

        while let Some(box1) = new_head1 {
            vals1.push(box1.val);
            new_head1 = box1.next;
        }
        while let Some(box2) = new_head2 {
            vals2.push(box2.val);
            new_head2 = box2.next;
        }

        vals1.reverse();
        vals2.reverse();

        let mut max_length = vals1.len() as i32;
        let min_length = vals2.len() as i32;

        if min_length > max_length {
            max_length = min_length;
        }

        for _ in 0..max_length {
            let first_number_option = vals1.pop();
            let second_number_option = vals2.pop();
            let mut first_number = 0;
            let mut second_number = 0;

            if !first_number_option.is_none() {
                first_number = first_number_option.unwrap();
            }
            if !second_number_option.is_none() {
                second_number = second_number_option.unwrap();
            }

            let sum = leading + first_number + second_number;
            raw_vect_result.push(sum % 10);
            leading = sum / 10;
        }

        if leading != 0 {
            raw_vect_result.push(leading);
        }

        raw_vect_result.reverse();

        for num in raw_vect_result {
            result = Some(Box::new(ListNode {
                val: num as i32,
                next: result,
            }))
        }

        if result.is_none() {
            return Some(Box::new(ListNode::new(0)));
        };
        return result;
    }
}

fn main() {
    let head1 = Some(Box::new(ListNode {
        val: 9,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode { val: 9, next: None })),
                        })),
                    })),
                })),
            })),
        })),
    }));
    let head2 = Some(Box::new(ListNode {
        val: 9,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode { val: 9, next: None })),
            })),
        })),
    }));

    let result = Solution::add_two_numbers(head1, head2);

    println!("{:?}", result);
}
