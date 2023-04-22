// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution {}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list_vals: Vec<i32> = vec![];
        let mut first_node: Option<Box<ListNode>> = None;

        while let Some(box_list1) = list1.take() {
            list_vals.push(box_list1.val);
            list1 = box_list1.next;
        }
        while let Some(box_list2) = list2.take() {
            list_vals.push(box_list2.val);
            list2 = box_list2.next;
        }

        list_vals.sort();
        list_vals.reverse();

        for i in 0..list_vals.len() {
            first_node = Some(Box::new(ListNode {
                val: list_vals[i],
                next: first_node.take(),
            }));
        }

        return first_node;
    }
}
fn main() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let list2 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(4))),
        })),
    }));

    let res = Solution::merge_two_lists(list1, list2);
    println!("{:?}", res);
}
