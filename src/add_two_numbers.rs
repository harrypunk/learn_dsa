// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, x) | (x, None) => x,
            (Some(x), Some(y)) => {
                let (num1, carry1) = add(x.val,y.val, 0);
                let mut carry = carry1;
                let mut head: Box<ListNode> = Box::new(ListNode::new(num1));
                let mut tail = &head;
                let mut left = &x.next;
                let mut right = &y.next;
                'calc: loop {
                    match (left, right) {
                        (None, None) => break 'calc,
                        (None, )
                    }
                }
                Some(head)
            }
        }
    }
}

//add two digits (num, carry)
fn add(a:i32, b:i32, carry: i32) -> (i32, i32) {
    let sum = a+b+carry;
    (sum%10, sum/10)
}