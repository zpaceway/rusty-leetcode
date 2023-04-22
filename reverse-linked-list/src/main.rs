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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head: Option<Box<ListNode>> = None;
        let mut last_head = head;
        while let Some(box_head) = last_head {
            new_head = Some(Box::new(ListNode {
                val: box_head.val,
                next: new_head,
            }));

            last_head = box_head.next;
        }

        return new_head;
    }
}

fn main() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    let new_head = Solution::reverse_list(head);
    println!("{:?}", new_head);
}
